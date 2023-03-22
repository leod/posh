use glam::{Vec3Swizzles, Vec4Swizzles};
use nanorand::{Rng, WyRand};

use posh::{gl, sl, Block, BlockDom, Gl, Sl};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const DEPTH_MAP_SIZE: u32 = 1024;
const NUM_CUBES: u32 = 10;

// Shader interface

#[derive(Clone, Copy, Block)]
pub struct Camera<D: BlockDom = Sl> {
    pub world_to_eye: D::Mat4,
    pub eye_to_clip: D::Mat4,
}

impl Camera<Sl> {
    pub fn world_to_clip_pos(self, world_pos: sl::Vec3) -> sl::Vec4 {
        self.eye_to_clip * (self.world_to_eye * world_pos.extend(1.0))
    }

    pub fn world_to_clip_normal(self, world_normal: sl::Vec3) -> sl::Vec3 {
        ((self.eye_to_clip * self.world_to_eye).transpose().inverse() * world_normal.extend(0.0))
            .xyz()
            .normalize()
    }
}

#[derive(Clone, Copy, Block)]
pub struct Light<D: BlockDom = Sl> {
    pub camera: Camera<D>,
    pub world_pos: D::Vec3,
    pub color: D::Vec3,
    pub ambient: D::Vec3,
}

#[derive(Clone, Copy, Block)]
pub struct SceneVertex<D: BlockDom = Sl> {
    pub world_pos: D::Vec3,
    pub world_normal: D::Vec3,
    pub color: D::Vec3,
}

// Shaders

mod flat_pass {
    use posh::sl;

    use super::{Camera, SceneVertex};

    pub fn vertex(camera: Camera, vertex: SceneVertex) -> sl::VaryingOutput<sl::Vec3> {
        sl::VaryingOutput {
            output: vertex.color,
            position: camera.world_to_clip_pos(vertex.world_pos),
        }
    }

    pub fn fragment(_: (), color: sl::Vec3) -> sl::Vec4 {
        color.extend(1.0)
    }
}

mod depth_pass {
    use posh::sl;

    use super::{Light, SceneVertex};

    pub fn vertex(light: Light, vertex: SceneVertex) -> sl::Vec4 {
        light.camera.world_to_clip_pos(vertex.world_pos)
    }

    pub fn fragment(_: (), _: ()) -> () {
        ()
    }
}

mod shaded_pass {
    use posh::{
        sl::{self, Value, Varying},
        Sl, UniformDom,
    };

    use super::{Camera, Light, SceneVertex};

    #[derive(Clone, posh::Uniform)]
    pub struct Uniform<D: UniformDom = Sl> {
        pub camera: D::Block<Camera>,
        pub light: D::Block<Light>,
        pub light_depth_map: D::ComparisonSampler2d,
    }

    #[derive(Clone, Copy, Value, Varying)]
    pub struct OutputVertex {
        world_pos: sl::Vec3,
        world_normal: sl::Vec3,
        color: sl::Vec3,
        light_clip_pos: sl::Vec4,
        light_clip_normal: sl::Vec3,
    }

    pub fn vertex(
        Uniform { light, camera, .. }: Uniform,
        vertex: SceneVertex,
    ) -> sl::VaryingOutput<OutputVertex> {
        let light_clip_pos = light
            .camera
            .world_to_clip_pos(vertex.world_pos + vertex.world_normal * 0.1);
        let light_clip_normal = light.camera.world_to_clip_normal(vertex.world_normal);

        let output = OutputVertex {
            world_pos: vertex.world_pos,
            world_normal: vertex.world_normal,
            color: vertex.color,
            light_clip_pos,
            light_clip_normal,
        };

        sl::VaryingOutput {
            output,
            position: camera.world_to_clip_pos(vertex.world_pos),
        }
    }

    fn sample_shadow(
        light_depth_map: sl::ComparisonSampler2d,
        light_clip_pos: sl::Vec4,
        light_clip_normal: sl::Vec3,
    ) -> sl::F32 {
        let ndc = light_clip_pos.xyz() / light_clip_pos.w;
        let norm = ndc.xy().dot(ndc.xy());
        let uvw = ndc * 0.5 + 0.5;
        let is_outside = norm.ge(1.0);
        (-1.0 * light_clip_normal.z).clamp(0.0, 1.0)
            * is_outside.branch(0.0, light_depth_map.sample_compare(uvw.xy(), uvw.z))
    }

    pub fn fragment(
        Uniform {
            light,
            light_depth_map,
            ..
        }: Uniform,
        vertex: OutputVertex,
    ) -> sl::Vec4 {
        let shadow = sample_shadow(
            light_depth_map,
            vertex.light_clip_pos,
            vertex.light_clip_normal,
        );

        //let world_normal = vertex.world_normal.normalize();
        let light_dir = (light.world_pos - vertex.world_pos).normalize();
        let diffuse = light.color * vertex.world_normal.dot(light_dir).max(0.0);

        let color = (light.ambient + shadow * diffuse) * vertex.color;

        color.extend(1.0)
    }
}

mod debug_pass {
    use posh::{sl, Block, BlockDom, Sl};

    #[derive(Clone, Copy, Block)]
    pub struct Vertex<D: BlockDom = Sl> {
        pub pos: D::Vec2,
        pub tex_coords: D::Vec2,
    }

    pub fn vertex(_: (), vertex: Vertex) -> sl::VaryingOutput<sl::Vec2> {
        sl::VaryingOutput {
            output: vertex.tex_coords,
            position: vertex.pos.extend(0.0).extend(1.0),
        }
    }

    pub fn fragment(sampler: sl::ColorSampler2d<sl::F32>, tex_coords: sl::Vec2) -> sl::Vec4 {
        let depth = sampler.sample(tex_coords);

        sl::Vec4::splat(depth)
    }
}

// Host code

struct Demo {
    ctx: gl::Context,

    flat_program: gl::Program<Camera, SceneVertex>,
    depth_program: gl::Program<Light, SceneVertex, ()>,
    shaded_program: gl::Program<shaded_pass::Uniform, SceneVertex>,
    debug_program: gl::Program<sl::ColorSampler2d<sl::F32>, debug_pass::Vertex>,

    camera_buffer: gl::UniformBuffer<Camera>,
    light_buffer: gl::UniformBuffer<Light>,
    light_depth_map: gl::DepthTexture2d,

    scene_vertices: gl::VertexBuffer<SceneVertex>,
    scene_elements: gl::ElementBuffer,

    light_vertices: gl::VertexBuffer<SceneVertex>,
    light_elements: gl::ElementBuffer,

    debug_vertices: gl::VertexBuffer<debug_pass::Vertex>,
    debug_elements: gl::ElementBuffer,
}

impl Demo {
    pub fn new(ctx: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::{StaticDraw, StreamDraw};

        let depth_map_size = glam::uvec2(DEPTH_MAP_SIZE, DEPTH_MAP_SIZE);
        let light_depth_image = gl::DepthImage::zero_f32(depth_map_size);

        Ok(Demo {
            ctx: ctx.clone(),

            flat_program: ctx.create_program(flat_pass::vertex, flat_pass::fragment)?,
            depth_program: ctx.create_program(depth_pass::vertex, depth_pass::fragment)?,
            shaded_program: ctx.create_program(shaded_pass::vertex, shaded_pass::fragment)?,
            debug_program: ctx.create_program(debug_pass::vertex, debug_pass::fragment)?,

            camera_buffer: ctx.create_uniform_buffer(Camera::default(), StaticDraw)?,
            light_buffer: ctx.create_uniform_buffer(Light::new(glam::Vec3::ZERO), StreamDraw)?,
            light_depth_map: ctx.create_depth_texture_2d(light_depth_image)?,

            scene_vertices: ctx.create_vertex_buffer(&scene_vertices(), StaticDraw)?,
            scene_elements: ctx.create_element_buffer(&scene_elements(), StaticDraw)?,

            light_vertices: ctx.create_vertex_buffer(&Vec::new(), StreamDraw)?,
            light_elements: ctx.create_element_buffer(&light_elements(), StaticDraw)?,

            debug_vertices: ctx.create_vertex_buffer(&debug_vertices(), StaticDraw)?,
            debug_elements: ctx.create_element_buffer(&debug_elements(), StaticDraw)?,
        })
    }

    pub fn draw(&mut self, mouse_pos: glam::UVec2) -> Result<(), gl::DrawError> {
        let light_pos = light_pos(mouse_pos);

        self.light_buffer.set(Light::new(light_pos));
        self.light_vertices.set(&light_vertices(light_pos));

        let scene_stream = gl::PrimitiveStream {
            vertices: self.scene_vertices.as_binding(),
            elements: self.scene_elements.as_binding(),
            mode: gl::Mode::Triangles,
        };

        self.depth_program.draw(
            self.light_buffer.as_binding(),
            scene_stream.clone(),
            self.light_depth_map.as_depth_attachment(),
            gl::DrawParams {
                clear_depth: Some(1.0),
                depth_compare: Some(gl::CompareFunction::Less),
                cull_face: Some(gl::CullFace::Back),
                ..Default::default()
            },
        )?;

        self.shaded_program.draw(
            shaded_pass::Uniform {
                camera: self.camera_buffer.as_binding(),
                light: self.light_buffer.as_binding(),
                light_depth_map: self.light_depth_map.as_comparison_sampler(
                    gl::Sampler2dParams::default(),
                    gl::CompareFunction::Less,
                ),
            },
            scene_stream,
            gl::DefaultFramebuffer::default(),
            gl::DrawParams {
                clear_color: Some(glam::Vec4::ONE),
                clear_depth: Some(1.0),
                depth_compare: Some(gl::CompareFunction::Less),
                cull_face: Some(gl::CullFace::Back),
                ..Default::default()
            },
        )?;

        self.flat_program.draw(
            self.camera_buffer.as_binding(),
            gl::PrimitiveStream {
                vertices: self.light_vertices.as_binding(),
                elements: self.light_elements.as_binding(),
                mode: gl::Mode::Triangles,
            },
            gl::DefaultFramebuffer::default(),
            gl::DrawParams {
                depth_compare: Some(gl::CompareFunction::Less),
                cull_face: Some(gl::CullFace::Back),
                ..Default::default()
            },
        )?;

        self.debug_program.draw(
            self.light_depth_map
                .as_color_sampler(gl::Sampler2dParams::default()),
            gl::PrimitiveStream {
                vertices: self.debug_vertices.as_binding(),
                elements: self.debug_elements.as_binding(),
                mode: gl::Mode::Triangles,
            },
            gl::DefaultFramebuffer::default(),
            gl::DrawParams::default(),
        )?;

        Ok(())
    }
}

// Scene data

const CAMERA_WORLD_POS: glam::Vec3 = glam::vec3(-5.0, 5.0, 10.0);
const LIGHT_WORLD_POS: glam::Vec3 = glam::vec3(0.0, 5.0, 5.0);
const LIGHT_CENTER_Z: f32 = -20.0;

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            world_to_eye: glam::Mat4::look_at_rh(
                CAMERA_WORLD_POS,
                glam::vec3(0.0, 0.0, LIGHT_CENTER_Z),
                glam::vec3(0.0, 1.0, 0.0),
            ),
            eye_to_clip: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
                1.0,
                100.0,
            ),
        }
    }
}

impl Light<Gl> {
    pub fn new(center: glam::Vec3) -> Self {
        Self {
            camera: Camera {
                world_to_eye: glam::Mat4::look_at_rh(
                    LIGHT_WORLD_POS,
                    center,
                    glam::vec3(0.0, 0.0, 1.0),
                ),
                eye_to_clip: glam::Mat4::orthographic_rh_gl(
                    -40.0, 40.0, -40.0, 40.0, -100.0, 100.0,
                ),
            },
            world_pos: LIGHT_WORLD_POS,
            color: glam::vec3(1.0, 1.0, 0.7),
            ambient: glam::vec3(0.1, 0.1, 0.1),
        }
    }
}

fn scene_vertices() -> Vec<SceneVertex<Gl>> {
    let mut rng = WyRand::new();

    (0..NUM_CUBES)
        .flat_map(|_| {
            let center = ((glam::vec2(rng.generate(), rng.generate()) - 0.5) * 30.0).extend(-15.0);
            let color = glam::vec3(rng.generate(), rng.generate(), rng.generate());
            let size = glam::Vec3::splat(4.0);

            cube_vertices(center, color, size, false)
        })
        .chain(cube_vertices(
            glam::vec3(0.0, 0.0, 0.0),
            glam::vec3(0.9, 0.9, 0.9),
            glam::Vec3::splat(50.0),
            true,
        ))
        .collect()
}

fn scene_elements() -> Vec<u32> {
    (0..NUM_CUBES + 1).flat_map(cube_elements).collect()
}

fn light_vertices(center_pos: glam::Vec3) -> Vec<SceneVertex<Gl>> {
    cube_vertices(
        LIGHT_WORLD_POS,
        glam::vec3(0.5, 0.5, 0.1),
        glam::Vec3::splat(2.0),
        false,
    )
    .chain(cube_vertices(
        center_pos,
        glam::vec3(0.5, 0.5, 0.1),
        glam::Vec3::splat(1.3),
        false,
    ))
    .collect()
}

fn light_elements() -> Vec<u32> {
    (0..2).flat_map(cube_elements).collect()
}

fn light_pos(mouse_pos: glam::UVec2) -> glam::Vec3 {
    let camera = Camera::default();
    let world_pos = CAMERA_WORLD_POS;

    let dir_ndc = glam::vec3(
        (2.0 * mouse_pos.x as f32) / SCREEN_WIDTH as f32 - 1.0,
        1.0 - (2.0 * mouse_pos.y as f32) / SCREEN_HEIGHT as f32,
        1.0,
    );
    let dir_clip = dir_ndc.xy().extend(-1.0).extend(1.0);
    let dir_eye = (camera.eye_to_clip.inverse() * dir_clip)
        .xy()
        .extend(-1.0)
        .extend(0.0);
    let dir_world = (camera.world_to_eye.inverse() * dir_eye).xyz().normalize();

    let t = (LIGHT_CENTER_Z - world_pos.z) / dir_world.z;

    world_pos + t * dir_world.xyz()
}

fn cube_vertices(
    center: glam::Vec3,
    color: glam::Vec3,
    size: glam::Vec3,
    invert: bool,
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
        world_pos: center + glam::Vec3::from(pos) * size * if invert { -1.0 } else { 1.0 },
        world_normal: glam::Vec3::from(
            [
                [1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, -1.0],
            ][i / 4],
        ),
        color,
    })
}

fn cube_elements(n: u32) -> impl Iterator<Item = u32> {
    let start = 24 * n;

    (0..6u32).flat_map(move |face| [1, 0, 2, 2, 0, 3].map(|i| start + face * 4 + i))
}

fn debug_vertices() -> Vec<debug_pass::Vertex<Gl>> {
    use debug_pass::Vertex;

    vec![
        Vertex {
            pos: [-1.0, -1.0].into(),
            tex_coords: [0.0, 0.0].into(),
        },
        Vertex {
            pos: [-0.5, -1.0].into(),
            tex_coords: [1.0, 0.0].into(),
        },
        Vertex {
            pos: [-0.5, -0.5].into(),
            tex_coords: [1.0, 1.0].into(),
        },
        Vertex {
            pos: [-1.0, -0.5].into(),
            tex_coords: [0.0, 1.0].into(),
        },
    ]
}

fn debug_elements() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3]
}

// SDL glue

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Simple shadow mapping", 1024, 768)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let ctx = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let ctx = gl::Context::new(ctx).unwrap();
    let mut demo = Demo::new(ctx).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();
    let mut mouse_pos = glam::UVec2::ZERO;

    loop {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            match event {
                Quit { .. } => {
                    return;
                }
                MouseMotion { x, y, .. } => {
                    mouse_pos.x = (x.max(0) as u32).min(SCREEN_WIDTH);
                    mouse_pos.y = (y.max(0) as u32).min(SCREEN_HEIGHT);
                }
                _ => (),
            }
        }

        demo.draw(mouse_pos).unwrap();
        window.gl_swap_window();
    }
}
