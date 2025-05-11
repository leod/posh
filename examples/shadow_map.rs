mod utils;

use instant::Instant;
use nanorand::{Rng, WyRand};

use posh::{gl, sl, Block, BlockDom, Gl, Sl, Uniform, UniformDom};

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const DEPTH_MAP_SIZE: u32 = 2048;
const NUM_CUBES: u32 = 30;

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct Camera<D: BlockDom> {
    pub world_to_eye: D::Mat4,
    pub eye_to_clip: D::Mat4,
}

impl Camera<Sl> {
    pub fn world_to_clip(self, world_pos: sl::Vec3) -> sl::Vec4 {
        self.eye_to_clip * (self.world_to_eye * world_pos.extend(1.0))
    }
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct Light<D: BlockDom> {
    pub camera: Camera<D>,
    pub world_pos: D::Vec3,
    pub color: D::Vec3,
    pub ambient: D::Vec3,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct SceneVertex<D: BlockDom> {
    pub world_pos: D::Vec3,
    pub world_normal: D::Vec3,
    pub color: D::Vec3,
}

#[derive(Clone, Uniform)]
pub struct SceneUniforms<D: UniformDom> {
    pub camera: D::Block<Camera<Sl>>,
    pub light: D::Block<Light<Sl>>,
    pub light_depth_map: D::ComparisonSampler2d,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct ScreenVertex<D: BlockDom> {
    pub pos: D::Vec2,
    pub tex_coords: D::Vec2,
}

// Shaders

mod flat_pass {
    use posh::{sl, Sl};

    use super::{Camera, SceneVertex};

    pub fn vertex_shader(camera: Camera<Sl>, vertex: SceneVertex<Sl>) -> sl::VsOutput<sl::Vec3> {
        sl::VsOutput {
            clip_pos: camera.world_to_clip(vertex.world_pos),
            interp: vertex.color,
        }
    }

    pub fn fragment_shader(color: sl::Vec3) -> sl::Vec4 {
        color.extend(1.0)
    }
}

mod scene_pass {
    use posh::{sl, Sl};

    use super::{SceneUniforms, SceneVertex};

    #[derive(Clone, Copy, sl::Value, sl::Interpolant)]
    pub struct Interpolant {
        vertex: SceneVertex<Sl>,
        light_clip_pos: sl::Vec4,
    }

    pub fn vertex_shader(
        SceneUniforms { light, camera, .. }: SceneUniforms<Sl>,
        vertex: SceneVertex<Sl>,
    ) -> sl::VsOutput<Interpolant> {
        const EXTRUDE: f32 = 0.1;

        let light_clip_pos = light
            .camera
            .world_to_clip(vertex.world_pos + vertex.world_normal * EXTRUDE);

        let output = Interpolant {
            vertex,
            light_clip_pos,
        };

        sl::VsOutput {
            clip_pos: camera.world_to_clip(vertex.world_pos),
            interp: output,
        }
    }

    fn sample_shadow(
        light_depth_map: sl::ComparisonSampler2d,
        light_clip_pos: sl::Vec4,
    ) -> sl::F32 {
        let ndc = light_clip_pos.xyz() / light_clip_pos.w;
        let uvw = ndc * 0.5 + 0.5;

        let is_inside = sl::all([uvw.x.ge(0.0), uvw.x.le(1.0), uvw.y.ge(0.0), uvw.y.le(1.0)]);

        is_inside.branch(light_depth_map.sample_compare(uvw.xy(), uvw.z), 0.0)
    }

    pub fn fragment_shader(
        SceneUniforms {
            light,
            light_depth_map,
            ..
        }: SceneUniforms<Sl>,
        Interpolant {
            vertex,
            light_clip_pos,
        }: Interpolant,
    ) -> sl::Vec4 {
        let light_dir = (light.world_pos - vertex.world_pos).normalize();
        let diffuse = light.color * vertex.world_normal.dot(light_dir).max(0.0);

        let shadow = sample_shadow(light_depth_map, light_clip_pos);

        let color = (light.ambient + shadow * diffuse) * vertex.color;

        color.extend(1.0)
    }
}

mod debug_pass {
    use posh::{sl, Sl};

    use crate::ScreenVertex;

    pub fn vertex_shader(vertex: ScreenVertex<Sl>) -> sl::VsOutput<sl::Vec2> {
        sl::VsOutput {
            interp: vertex.tex_coords,
            clip_pos: vertex.pos.extend(0.0).extend(1.0),
        }
    }

    pub fn fragment_shader(sampler: sl::ColorSampler2d<sl::F32>, tex_coords: sl::Vec2) -> sl::Vec4 {
        let depth = sampler.sample(tex_coords);

        sl::Vec4::splat(depth)
    }
}

// Host code

struct Demo {
    gl: gl::Context,

    camera_buffer: gl::UniformBuffer<Camera<Gl>>,
    light_buffer: gl::UniformBuffer<Light<Gl>>,
    light_depth_map: gl::DepthTexture2d,

    scene_vertices: gl::VertexBuffer<SceneVertex<Gl>>,
    scene_elements: gl::ElementBuffer,

    light_vertices: gl::VertexBuffer<SceneVertex<Gl>>,
    light_elements: gl::ElementBuffer,

    debug_vertices: gl::VertexBuffer<ScreenVertex<Gl>>,
    debug_elements: gl::ElementBuffer,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::{StaticDraw, StreamDraw};

        let light_depth_image =
            gl::DepthImage::u24_depth_u8_stencil_zero([DEPTH_MAP_SIZE, DEPTH_MAP_SIZE]);

        Ok(Demo {
            gl: gl.clone(),

            camera_buffer: gl.create_uniform_buffer(Camera::default(), StaticDraw)?,
            light_buffer: gl.create_uniform_buffer(Light::new(0.0, 0.0), StreamDraw)?,
            light_depth_map: gl.create_depth_texture_2d(light_depth_image)?,

            scene_vertices: gl.create_vertex_buffer(&scene_vertices(), StaticDraw)?,
            scene_elements: gl.create_element_buffer(&scene_elements(), StaticDraw)?,

            light_vertices: gl.create_vertex_buffer(&Vec::new(), StreamDraw)?,
            light_elements: gl.create_element_buffer(&light_elements(), StaticDraw)?,

            debug_vertices: gl.create_vertex_buffer(&debug_vertices(), StaticDraw)?,
            debug_elements: gl.create_element_buffer(&debug_elements(), StaticDraw)?,

            start_time: Instant::now(),
        })
    }

    pub fn draw(&mut self) -> Result<(), gl::DrawError> {
        let dt = Instant::now().duration_since(self.start_time).as_secs_f32();

        let light_x = (dt * 0.5).sin() * 20.0;
        let light_y = 20.0 + dt.sin() * 10.0;

        self.light_buffer.set(Light::new(light_x, light_y));
        self.light_vertices.set(&light_vertices(light_x, light_y));

        let scene_vertex_spec = self
            .scene_vertices
            .as_vertex_spec(gl::PrimitiveMode::Triangles)
            .with_element_data(self.scene_elements.as_binding());

        self.gl.clear(
            self.light_depth_map.as_depth_attachment(),
            gl::ClearParams {
                depth: Some(1.0),
                ..Default::default()
            },
        )?;

        self.gl
            .program(
                |light: Light<Sl>, vertex: SceneVertex<Sl>| {
                    light.camera.world_to_clip(vertex.world_pos)
                },
                |()| (),
            )
            .with_uniforms(self.light_buffer.as_binding())
            .with_framebuffer(self.light_depth_map.as_depth_attachment())
            .with_params(
                gl::DrawParams::new()
                    .with_depth_test(gl::Comparison::Less)
                    .with_cull_face(gl::CullFace::Back),
            )
            .draw(scene_vertex_spec.clone())?;

        self.gl.clear(
            gl::Framebuffer::default(),
            gl::ClearParams {
                color: Some(glam::Vec4::ONE.into()),
                depth: Some(1.0),
                ..Default::default()
            },
        )?;

        self.gl
            .program(scene_pass::vertex_shader, scene_pass::fragment_shader)
            .with_uniforms(SceneUniforms {
                camera: self.camera_buffer.as_binding(),
                light: self.light_buffer.as_binding(),
                light_depth_map: self
                    .light_depth_map
                    .as_comparison_sampler(gl::Sampler2dParams::linear(), gl::Comparison::Less),
            })
            .with_params(
                gl::DrawParams::new()
                    .with_depth_test(gl::Comparison::Less)
                    .with_cull_face(gl::CullFace::Back),
            )
            .draw(scene_vertex_spec.clone())?;

        self.gl
            .program(flat_pass::vertex_shader, flat_pass::fragment_shader)
            .with_uniforms(self.camera_buffer.as_binding())
            .with_params(
                gl::DrawParams::new()
                    .with_depth_test(gl::Comparison::Less)
                    .with_cull_face(gl::CullFace::Back),
            )
            .draw(
                self.light_vertices
                    .as_vertex_spec(gl::PrimitiveMode::Triangles)
                    .with_element_data(self.light_elements.as_binding()),
            )?;

        self.gl
            .program(debug_pass::vertex_shader, debug_pass::fragment_shader)
            .with_uniforms(
                self.light_depth_map
                    .as_color_sampler(gl::Sampler2dParams::default()),
            )
            .draw(
                self.debug_vertices
                    .as_vertex_spec(gl::PrimitiveMode::Triangles)
                    .with_element_data(self.debug_elements.as_binding()),
            )?;

        Ok(())
    }
}

// Scene data

const CAMERA_POS: glam::Vec3 = glam::vec3(0.0, 40.0, 40.0);
const CUBE_SIZE: f32 = 4.0;
const FLOOR_POS: glam::Vec3 = glam::vec3(0.0, -2.5, 0.0);
const FLOOR_SIZE: glam::Vec3 = glam::vec3(200.0, 5.0, 100.0);

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            world_to_eye: glam::Mat4::look_at_rh(
                CAMERA_POS,
                glam::Vec3::new(0.0, 0.0, 10.0),
                glam::vec3(0.0, 1.0, 0.0),
            )
            .into(),
            eye_to_clip: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
                1.0,
                100.0,
            )
            .into(),
        }
    }
}

impl Light<Gl> {
    pub fn new(world_x: f32, world_y: f32) -> Self {
        let world_pos = glam::vec3(world_x, world_y, 35.0);

        Self {
            camera: Camera {
                world_to_eye: glam::Mat4::look_at_rh(
                    world_pos,
                    glam::vec3(0.0, 0.0, 0.0),
                    glam::vec3(0.0, 1.0, 0.0),
                )
                .into(),
                eye_to_clip: glam::Mat4::perspective_rh_gl(
                    std::f32::consts::PI / 1.5,
                    1.0,
                    10.0,
                    150.0,
                )
                .into(),
            },
            world_pos: world_pos.into(),
            color: glam::vec3(1.0, 1.0, 0.7).into(),
            ambient: glam::vec3(0.1, 0.1, 0.1).into(),
        }
    }
}

fn scene_vertices() -> Vec<SceneVertex<Gl>> {
    let mut rng = WyRand::new();

    (0..NUM_CUBES)
        .flat_map(|_| {
            let center = glam::vec3(
                (rng.generate::<f32>() - 0.5) * FLOOR_SIZE.x,
                CUBE_SIZE / 2.0,
                -rng.generate::<f32>() * 60.0 + 25.0,
            );
            let size = glam::Vec3::splat(CUBE_SIZE);
            let color = glam::vec3(rng.generate(), rng.generate(), rng.generate());

            rect_cuboid_vertices(center, size, color)
        })
        .chain(rect_cuboid_vertices(
            FLOOR_POS,
            FLOOR_SIZE,
            glam::vec3(0.9, 0.9, 0.9),
        ))
        .collect()
}

fn scene_elements() -> Vec<u32> {
    (0..NUM_CUBES + 1).flat_map(cuboid_elements).collect()
}

fn light_vertices(x: f32, y: f32) -> Vec<SceneVertex<Gl>> {
    let world_pos = Light::new(x, y).world_pos;

    rect_cuboid_vertices(
        world_pos.into(),
        glam::Vec3::splat(2.0),
        glam::vec3(0.5, 0.5, 0.1),
    )
    .collect()
}

fn light_elements() -> Vec<u32> {
    cuboid_elements(0).collect()
}

fn rect_cuboid_vertices(
    center: glam::Vec3,
    size: glam::Vec3,
    color: glam::Vec3,
) -> impl Iterator<Item = SceneVertex<Gl>> {
    [
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
    ]
    .into_iter()
    .enumerate()
    .map(move |(i, pos)| SceneVertex {
        world_pos: (center + glam::Vec3::from(pos) * size).into(),
        world_normal: glam::Vec3::from(
            [
                [1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, -1.0],
            ][i / 4],
        )
        .into(),
        color: color.into(),
    })
}

fn cuboid_elements(n: u32) -> impl Iterator<Item = u32> {
    let start = 24 * n;

    (0..6u32).flat_map(move |face| [1, 0, 2, 2, 0, 3].map(|i| start + face * 4 + i))
}

fn debug_vertices() -> Vec<ScreenVertex<Gl>> {
    vec![
        ScreenVertex {
            pos: [-1.0, 1.0].into(),
            tex_coords: [0.0, 1.0].into(),
        },
        ScreenVertex {
            pos: [-0.5, 1.0].into(),
            tex_coords: [1.0, 1.0].into(),
        },
        ScreenVertex {
            pos: [-0.5, 0.5].into(),
            tex_coords: [1.0, 0.0].into(),
        },
        ScreenVertex {
            pos: [-1.0, 0.5].into(),
            tex_coords: [0.0, 0.0].into(),
        },
    ]
}

fn debug_elements() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3]
}

// Platform glue

fn main() {
    utils::run_demo("Simple shadow mapping", Demo::new, Demo::draw);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn run() {
    utils::init_wasm().await;

    #[allow(clippy::main_recursion)]
    main();
}
