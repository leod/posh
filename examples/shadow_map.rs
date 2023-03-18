use nanorand::{Rng, WyRand};

use posh::{gl, Block, BlockDom, Gl, Sl};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const NUM_CUBES: u32 = 1000;

// Shader interface

#[derive(Clone, Copy, Block)]
pub struct Camera<D: BlockDom = Sl> {
    pub projection: D::Mat4,
    pub view: D::Mat4,
}

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            projection: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                WIDTH as f32 / HEIGHT as f32,
                1.0,
                100.0,
            ),
            view: glam::Mat4::look_at_rh(
                glam::vec3(5.0, 20.0, -20.0),
                glam::Vec3::ZERO,
                glam::vec3(0.0, 1.0, 0.0),
            ),
        }
    }
}

#[derive(Clone, Copy, Block)]
pub struct Light<D: BlockDom = Sl> {
    pub camera: Camera<D>,
    pub pos: D::Vec3,
    pub color: D::Vec4,
}

impl Light<Gl> {
    pub fn new(pos: glam::Vec3) -> Self {
        Self {
            camera: Camera {
                projection: glam::Mat4::orthographic_rh_gl(-10.0, 10.0, -10.0, 0.0, 1.0, 7.5),
                view: glam::Mat4::look_at_rh(
                    glam::vec3(-2.0, 4.0, -1.0),
                    pos,
                    glam::vec3(0.0, 1.0, 0.0),
                ),
            },
            pos,
            color: glam::vec4(1.0, 1.0, 0.7, 1.0),
        }
    }
}

#[derive(Clone, Copy, Block)]
pub struct Vertex<D: BlockDom = Sl> {
    pub pos: D::Vec3,
    pub normal: D::Vec3,
    pub color: D::Vec4,
}

// Shaders

mod scene {
    use posh::{
        sl::{self, Value, Varying},
        Sl, Uniform, UniformDom,
    };

    use super::{Camera, Light, Vertex};

    #[derive(Clone, Copy, Value, Varying)]
    pub struct Varyings {
        normal: sl::Vec4,
        color: sl::Vec4,
    }

    #[derive(Uniform)]
    pub struct Uniforms<D: UniformDom = Sl> {
        pub player: D::Block<Camera>,
        pub light: D::Block<Light>,
        pub light_depth_map: D::ComparisonSampler2d,
    }

    pub fn vertex(uniform: Uniforms, input: Vertex) -> sl::VaryingOutput<Varyings> {
        let output = Varyings {
            normal: uniform.player.view * input.normal.extend(1.0), // TODO
            color: input.color,
        };
        let position = uniform.player.projection * uniform.player.view * input.pos.extend(1.0);

        sl::VaryingOutput { output, position }
    }

    pub fn fragment(uniform: Uniforms, input: Varyings) -> sl::Vec4 {
        input.color
    }
}

// Host code

struct Demo {
    scene_program: gl::Program<scene::Uniforms, Vertex>,

    player: gl::UniformBuffer<Camera>,
    light: gl::UniformBuffer<Light>,
    light_depth_map: gl::DepthTexture2d,

    vertices: gl::VertexBuffer<Vertex>,
    elements: gl::ElementBuffer,
}

impl Demo {
    pub fn new(context: gl::Context) -> Result<Self, gl::CreateError> {
        let scene_program = context.create_program(scene::vertex, scene::fragment)?;

        let player =
            context.create_uniform_buffer(Camera::default(), gl::BufferUsage::StaticDraw)?;
        let light = context
            .create_uniform_buffer(Light::new(glam::Vec3::ZERO), gl::BufferUsage::StaticDraw)?;
        let light_depth_map = context
            .create_depth_texture_2d(gl::DepthImage::zero_f32(glam::uvec2(WIDTH, HEIGHT)))?;

        let mut rng = WyRand::new();
        let vertices: Vec<_> = (0..NUM_CUBES)
            .flat_map(|_| {
                let center =
                    (glam::vec3(rng.generate(), rng.generate(), rng.generate()) - 0.5) * 50.0;
                let color = glam::vec4(rng.generate(), rng.generate(), rng.generate(), 1.0);

                cube_vertices(center, color)
            })
            .collect();
        let vertices = context.create_vertex_buffer(&vertices, gl::BufferUsage::StaticDraw)?;

        let elements: Vec<_> = (0..NUM_CUBES).flat_map(cube_elements).collect();
        let elements = context.create_element_buffer(&elements, gl::BufferUsage::StaticDraw)?;

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
            scene::Uniforms {
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
            gl::DrawParams::default()
                .with_clear_color(glam::Vec4::ONE)
                .with_clear_depth(1.0)
                .with_depth_compare(gl::CompareFunction::Less),
        )
    }
}

// Mesh data

fn cube_vertices(center: glam::Vec3, color: glam::Vec4) -> Vec<Vertex<Gl>> {
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
    .map(|(i, pos)| Vertex {
        pos: center + glam::Vec3::from(pos),
        normal: [0.0, 0.0, 1.0].into(), // TODO
        color,
    })
    .collect()
}

fn cube_elements(n: u32) -> Vec<u32> {
    let start = 24 * n;

    (0..6u32)
        .flat_map(|face| [0, 1, 2, 0, 2, 3].map(|i| start + face * 4 + i))
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
