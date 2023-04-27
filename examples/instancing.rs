use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use posh::{gl, sl, Block, BlockDom, Gl, Sl, VertexDom};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Camera<D: BlockDom = Sl> {
    world_to_view: D::Mat4,
    view_to_screen: D::Mat4,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Instance<D: BlockDom = Sl> {
    model_to_view: D::Mat4,
    color: D::Vec3,
}

#[derive(Copy, Clone, bytemuck::Zeroable, bytemuck::Pod)]
#[repr(C)]
struct Foo {
    x: [f32; 4],
    y: f32,
}

#[derive(posh::Vertex)]
struct Vertex<D: VertexDom = Sl> {
    instance: D::Block<Instance>,
    model_pos: D::Block<sl::Vec3>,
}

// Shader code

fn vertex_shader(camera: Camera, vertex: Vertex) -> sl::VertexOutput<sl::Vec3> {
    sl::VertexOutput {
        position: camera.view_to_screen
            * camera.world_to_view
            * vertex.instance.model_to_view
            * vertex.model_pos.extend(1.0),
        varying: vertex.instance.color,
    }
}

fn fragment_shader(_: (), color: sl::Vec3) -> sl::Vec4 {
    color.extend(1.0)
}

// Host code

struct Demo {
    program: gl::Program<Camera, Vertex>,

    camera: gl::UniformBuffer<Camera>,

    instances: gl::VertexBuffer<Instance>,
    teapot: gl::VertexBuffer<sl::Vec3>,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        Ok(Self {
            program: gl.create_program(vertex_shader, fragment_shader)?,
            camera: gl.create_uniform_buffer(Camera::default(), StaticDraw)?,
            instances: gl.create_vertex_buffer(&instances(0.0), StaticDraw)?,
            teapot: gl.create_vertex_buffer(&teapot_positions(), StaticDraw)?,
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        self.program.draw(
            gl::Input {
                uniform: &self.camera.as_binding(),
                vertex: &gl::VertexSpec::new(gl::Mode::Triangles).with_vertex_data(Vertex {
                    instance: self.instances.as_binding().with_instancing(),
                    model_pos: self.teapot.as_binding(),
                }),
                settings: &gl::Settings::default()
                    .with_clear_color([0.1, 0.2, 0.3, 1.0])
                    .with_clear_depth(1.0)
                    .with_depth_test(gl::Comparison::Less),
            },
            gl::Framebuffer::default(),
        )
    }
}

// SDL glue

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Teapot instancing", 1024, 768)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let gl = gl::Context::new(gl).unwrap();
    let demo = Demo::new(gl).unwrap();

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

// Scene data

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            world_to_view: glam::Mat4::look_at_rh(
                glam::Vec3::new(-20.0, -20.0, -20.0),
                glam::Vec3::ZERO,
                glam::Vec3::NEG_Y,
            )
            .into(),
            view_to_screen: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                WIDTH as f32 / HEIGHT as f32,
                1.0,
                500.0,
            )
            .into(),
        }
    }
}

fn instances(_time: f32) -> Vec<Instance<Gl>> {
    (0..10)
        .flat_map(|x| {
            (0..10).flat_map(move |y| {
                (0..10).map(move |z| {
                    let world_pos = glam::uvec3(x, y, z).as_vec3() * 10.;
                    let model_to_view = glam::Mat4::from_translation(world_pos);
                    let color = glam::uvec3(x, 10 - y, z).as_vec3() / 10.0;

                    Instance {
                        model_to_view: model_to_view.into(),
                        color: color.into(),
                    }
                })
            })
        })
        .collect()
}

fn teapot_positions() -> Vec<gl::Vec3> {
    let file = File::open("examples/resources/teapot.csv").expect("Could not find teapot.csv");
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let cols = line.split(",").collect::<Vec<_>>();
            assert_eq!(cols.len(), 3);

            [
                cols[0].parse().unwrap(),
                cols[1].parse().unwrap(),
                cols[2].parse().unwrap(),
            ]
            .into()
        })
        .collect()
}
