use std::time::Instant;

use nanorand::{Rng, WyRand};

use posh::{gl, sl, Block, BlockDom, Gl, Sl};

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;
const DEPTH_MAP_SIZE: u32 = 2048;
const NUM_CUBES: u32 = 30;

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct Camera<D: BlockDom = Sl> {
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
pub struct Light<D: BlockDom = Sl> {
    pub camera: Camera<D>,
    pub world_pos: D::Vec3,
    pub color: D::Vec3,
    pub ambient: D::Vec3,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct SceneVertex<D: BlockDom = Sl> {
    pub world_pos: D::Vec3,
    pub world_normal: D::Vec3,
    pub color: D::Vec3,
}

// Shaders

mod flat_pass {
    use posh::sl;

    use super::{Camera, SceneVertex};

    pub fn vertex(camera: Camera, vertex: SceneVertex) -> sl::VertexOutput<sl::Vec3> {
        sl::VertexOutput {
            position: camera.world_to_clip(vertex.world_pos),
            varying: vertex.color,
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
        light.camera.world_to_clip(vertex.world_pos)
    }

    pub fn fragment(_: (), _: ()) -> () {
        ()
    }
}

mod shaded_pass {
    use posh::{sl, Sl, UniformDom};

    use super::{Camera, Light, SceneVertex};

    #[derive(Clone, posh::Uniform)]
    pub struct Uniform<D: UniformDom = Sl> {
        pub camera: D::Block<Camera>,
        pub light: D::Block<Light>,
        pub light_depth_map: D::ComparisonSampler2d,
    }

    #[derive(Clone, Copy, sl::Value, sl::Varying)]
    pub struct Varying {
        vertex: SceneVertex,
        light_clip_pos: sl::Vec4,
    }

    pub fn vertex(
        Uniform { light, camera, .. }: Uniform,
        vertex: SceneVertex,
    ) -> sl::VertexOutput<Varying> {
        const EXTRUDE: f32 = 0.1;

        let light_clip_pos = light
            .camera
            .world_to_clip(vertex.world_pos + vertex.world_normal * EXTRUDE);

        let output = Varying {
            vertex,
            light_clip_pos,
        };

        sl::VertexOutput {
            position: camera.world_to_clip(vertex.world_pos),
            varying: output,
        }
    }

    fn sample_shadow(
        light_depth_map: sl::ComparisonSampler2d,
        light_clip_pos: sl::Vec4,
    ) -> sl::F32 {
        let ndc = light_clip_pos.xyz() / light_clip_pos.w;
        let uvw = ndc * 0.5 + 0.5;

        let inside = !sl::any([uvw.x.lt(0.0), uvw.x.gt(1.0), uvw.y.lt(0.0), uvw.y.gt(1.0)]);

        sl::branch(inside, light_depth_map.sample_compare(uvw.xy(), uvw.z), 0.0)
    }

    pub fn fragment(
        Uniform {
            light,
            light_depth_map,
            ..
        }: Uniform,
        varying: Varying,
    ) -> sl::Vec4 {
        let light_dir = (light.world_pos - varying.vertex.world_pos).normalize();
        let diffuse = light.color * varying.vertex.world_normal.dot(light_dir).max(0.0);

        let shadow = sample_shadow(light_depth_map, varying.light_clip_pos);

        let color = (light.ambient + shadow * diffuse) * varying.vertex.color;

        color.extend(1.0)
    }
}

mod debug_pass {
    use posh::{sl, Block, BlockDom, Sl};

    #[derive(Clone, Copy, Block)]
    #[repr(C)]
    pub struct Vertex<D: BlockDom = Sl> {
        pub pos: D::Vec2,
        pub tex_coords: D::Vec2,
    }

    pub fn vertex(_: (), vertex: Vertex) -> sl::VertexOutput<sl::Vec2> {
        sl::VertexOutput {
            varying: vertex.tex_coords,
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

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::{StaticDraw, StreamDraw};

        let light_depth_image = gl::DepthImage::f32_zero([DEPTH_MAP_SIZE, DEPTH_MAP_SIZE]);

        Ok(Demo {
            flat_program: gl.create_program(flat_pass::vertex, flat_pass::fragment)?,
            depth_program: gl.create_program(depth_pass::vertex, depth_pass::fragment)?,
            shaded_program: gl.create_program(shaded_pass::vertex, shaded_pass::fragment)?,
            debug_program: gl.create_program(debug_pass::vertex, debug_pass::fragment)?,

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

        let scene_spec = self
            .scene_vertices
            .as_vertex_spec(gl::Mode::Triangles)
            .with_element_data(self.scene_elements.as_binding());

        self.depth_program.draw(
            gl::Input {
                uniform: &self.light_buffer.as_binding(),
                vertex: &scene_spec,
                settings: &gl::Settings::default()
                    .with_clear_depth(1.0)
                    .with_depth_test(gl::Comparison::Less)
                    .with_cull_face(gl::CullFace::Back),
            },
            self.light_depth_map.as_depth_attachment(),
        )?;

        self.shaded_program.draw(
            gl::Input {
                uniform: &shaded_pass::Uniform {
                    camera: self.camera_buffer.as_binding(),
                    light: self.light_buffer.as_binding(),
                    light_depth_map: self.light_depth_map.as_comparison_sampler(
                        gl::Sampler2dSettings::linear(),
                        gl::Comparison::Less,
                    ),
                },
                vertex: &scene_spec,
                settings: &gl::Settings::default()
                    .with_clear_color(glam::Vec4::ONE.into())
                    .with_clear_depth(1.0)
                    .with_depth_test(gl::Comparison::Less)
                    .with_cull_face(gl::CullFace::Back),
            },
            gl::Framebuffer::default(),
        )?;

        self.flat_program.draw(
            gl::Input {
                uniform: &self.camera_buffer.as_binding(),
                vertex: &self
                    .light_vertices
                    .as_vertex_spec(gl::Mode::Triangles)
                    .with_element_data(self.light_elements.as_binding()),
                settings: &gl::Settings::default()
                    .with_depth_test(gl::Comparison::Less)
                    .with_cull_face(gl::CullFace::Back),
            },
            gl::Framebuffer::default(),
        )?;

        self.debug_program.draw(
            gl::Input {
                uniform: &self
                    .light_depth_map
                    .as_color_sampler(gl::Sampler2dSettings::default()),
                vertex: &self
                    .debug_vertices
                    .as_vertex_spec(gl::Mode::Triangles)
                    .with_element_data(self.debug_elements.as_binding()),
                settings: &gl::Settings::default(),
            },
            gl::Framebuffer::default(),
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

fn debug_vertices() -> Vec<debug_pass::Vertex<Gl>> {
    use debug_pass::Vertex;

    vec![
        Vertex {
            pos: [-1.0, 1.0].into(),
            tex_coords: [0.0, 1.0].into(),
        },
        Vertex {
            pos: [-0.5, 1.0].into(),
            tex_coords: [1.0, 1.0].into(),
        },
        Vertex {
            pos: [-0.5, 0.5].into(),
            tex_coords: [1.0, 0.0].into(),
        },
        Vertex {
            pos: [-1.0, 0.5].into(),
            tex_coords: [0.0, 0.0].into(),
        },
    ]
}

fn debug_elements() -> Vec<u32> {
    vec![0, 1, 2, 0, 2, 3]
}

// SDL glue

fn main() {
    simple_logger::init().unwrap();

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Simple shadow mapping", SCREEN_WIDTH, SCREEN_HEIGHT)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let gl = gl::Context::new(gl).unwrap();
    let mut demo = Demo::new(gl).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();

    loop {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            if matches!(event, Quit { .. }) {
                return;
            }
        }

        demo.draw().unwrap();
        window.gl_swap_window();
    }
}
