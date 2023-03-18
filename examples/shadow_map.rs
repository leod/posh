use nanorand::{Rng, WyRand};

use posh::{gl, Block, BlockDom, Gl, Sl};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;
const DEPTH_MAP_SIZE: u32 = 1024;
const NUM_CUBES: u32 = 1000;

// Shader interface

#[derive(Clone, Copy, Block)]
pub struct Camera<D: BlockDom = Sl> {
    pub world_to_view: D::Mat4,
    pub view_to_screen: D::Mat4,
}

#[derive(Clone, Copy, Block)]
pub struct Light<D: BlockDom = Sl> {
    pub camera: Camera<D>,
    pub pos: D::Vec3,
    pub color: D::Vec3,
    pub ambient: D::Vec3,
}

#[derive(Clone, Copy, Block)]
pub struct Vertex<D: BlockDom = Sl> {
    pub pos: D::Vec3,
    pub normal: D::Vec3,
    pub color: D::Vec3,
}

// Shaders

mod scene_pass {
    use posh::{
        sl::{self, Value, Varying},
        Sl, UniformDom,
    };

    use super::{Camera, Light, Vertex};

    #[derive(Clone, Copy, Value, Varying)]
    pub struct OutputVertex {
        world_pos: sl::Vec3,
        normal: sl::Vec3,
        color: sl::Vec3,
    }

    #[derive(posh::Uniform)]
    pub struct Uniform<D: UniformDom = Sl> {
        pub player: D::Block<Camera>,
        pub light: D::Block<Light>,
        pub light_depth_map: D::ComparisonSampler2d,
    }

    pub fn vertex(uniform: Uniform, input: Vertex) -> sl::VaryingOutput<OutputVertex> {
        let output = OutputVertex {
            world_pos: input.pos,
            normal: input.normal,
            color: input.color,
        };
        let position =
            uniform.player.view_to_screen * uniform.player.world_to_view * input.pos.extend(1.0);

        sl::VaryingOutput { output, position }
    }

    pub fn fragment(uniform: Uniform, input: OutputVertex) -> sl::Vec4 {
        let light = uniform.light;

        let normal = input.normal.normalize();
        let light_dir = (light.pos - input.world_pos).normalize();
        let diffuse = light.color * normal.dot(light_dir).max(0.0);

        ((light.ambient + diffuse) * input.color).extend(1.0)
    }
}

// Host code

struct Demo {
    scene_program: gl::Program<scene_pass::Uniform, Vertex>,

    player: gl::UniformBuffer<Camera>,
    light: gl::UniformBuffer<Light>,
    light_depth_map: gl::DepthTexture2d,

    vertices: gl::VertexBuffer<Vertex>,
    elements: gl::ElementBuffer,
}

impl Demo {
    pub fn new(context: gl::Context) -> Result<Self, gl::CreateError> {
        let scene_program = context.create_program(scene_pass::vertex, scene_pass::fragment)?;

        let player =
            context.create_uniform_buffer(Camera::default(), gl::BufferUsage::StaticDraw)?;
        let light = context.create_uniform_buffer(
            Light::new(glam::vec3(0.0, 70.0, 0.0)),
            gl::BufferUsage::StreamDraw,
        )?;
        let light_depth_map = context.create_depth_texture_2d(gl::DepthImage::zero_f32(
            glam::uvec2(DEPTH_MAP_SIZE, DEPTH_MAP_SIZE),
        ))?;

        let vertices =
            context.create_vertex_buffer(&cube_vertices(), gl::BufferUsage::StaticDraw)?;
        let elements =
            context.create_element_buffer(&cube_elements(), gl::BufferUsage::StaticDraw)?;

        Ok(Demo {
            scene_program,
            player,
            light,
            light_depth_map,
            vertices,
            elements,
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        self.scene_program.draw(
            scene_pass::Uniform {
                player: self.player.as_binding(),
                light: self.light.as_binding(),
                light_depth_map: self.light_depth_map.as_comparison_sampler(
                    gl::Sampler2dParams::default(),
                    gl::CompareFunction::Less,
                ),
            },
            gl::VertexStream {
                vertices: self.vertices.as_binding(),
                elements: self.elements.as_binding(),
                primitive: gl::PrimitiveType::Triangles,
            },
            gl::DefaultFramebuffer::default(),
            gl::DrawParams {
                clear_color: Some(glam::Vec4::ONE),
                clear_depth: Some(1.0),
                depth_compare: Some(gl::CompareFunction::Less),
                ..Default::default()
            },
        )
    }
}

// Scene data

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            world_to_view: glam::Mat4::look_at_rh(
                glam::vec3(5.0, 20.0, -20.0),
                glam::Vec3::ZERO,
                glam::vec3(0.0, 1.0, 0.0),
            ),
            view_to_screen: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32,
                1.0,
                100.0,
            ),
        }
    }
}

impl Light<Gl> {
    pub fn new(pos: glam::Vec3) -> Self {
        Self {
            camera: Camera {
                world_to_view: glam::Mat4::look_at_rh(
                    pos,
                    pos - glam::vec3(0.0, 10.0, 0.0),
                    glam::vec3(0.0, 1.0, 0.0),
                ),
                view_to_screen: glam::Mat4::orthographic_rh_gl(-10.0, 10.0, -10.0, 0.0, 1.0, 7.5),
            },
            pos,
            color: glam::vec3(1.0, 1.0, 0.7),
            ambient: glam::vec3(0.1, 0.1, 0.1),
        }
    }
}

fn cube_vertices() -> Vec<Vertex<Gl>> {
    let mut rng = WyRand::new();

    (0..NUM_CUBES)
        .flat_map(|_| {
            let center = (glam::vec3(rng.generate(), rng.generate(), rng.generate()) - 0.5) * 30.0;
            let color = glam::vec3(rng.generate(), rng.generate(), rng.generate());
            let size = 1.0;

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
            .map(move |(i, pos)| Vertex {
                pos: center + glam::Vec3::from(pos) * size,
                normal: [
                    [1.0, 0.0, 0.0],
                    [-1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, -1.0, 0.0],
                    [0.0, 0.0, 1.0],
                    [0.0, 0.0, -1.0],
                ][i / 4]
                    .into(),
                color,
            })
        })
        .collect()
}

fn cube_elements() -> Vec<u32> {
    (0..NUM_CUBES)
        .flat_map(|n| {
            let start = 24 * n;

            (0..6u32).flat_map(move |face| [0, 1, 2, 0, 2, 3].map(|i| start + face * 4 + i))
        })
        .collect()
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
    let context = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let context = gl::Context::new(context).unwrap();
    let demo = Demo::new(context).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            if matches!(event, Quit { .. }) {
                running = false;
            }
        }

        demo.draw().unwrap();
        window.gl_swap_window();
    }
}
