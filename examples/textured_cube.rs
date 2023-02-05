use std::time::Instant;

use posh::{
    gl::{
        BufferUsage, Context, DefaultFramebuffer, DrawParams, Error, GeometryType, Program,
        UniformBuffer, VertexArray,
    },
    sl::{self, VaryingOutput},
    Block, BlockDomain, Sl, UniformDomain, UniformInterface,
};

// Shader interface

#[derive(Clone, Copy, Block, Default)]
struct Camera<D: BlockDomain = Sl> {
    projection: D::Mat4,
    view: D::Mat4,
}

#[derive(Clone, Copy, Block)]
struct Vertex<D: BlockDomain = Sl> {
    pos: D::Vec3<f32>,
    tex_coords: D::Vec2<f32>,
}

#[derive(UniformInterface)]
struct Uniforms<D: UniformDomain = Sl> {
    camera: D::Block<Camera>,
    sampler: D::Sampler2d<sl::Vec4<f32>>,
}

// Shader code

fn vertex_shader(uniforms: Uniforms, vertex: Vertex) -> VaryingOutput<sl::Vec2<f32>> {
    let camera = uniforms.camera;
    let position = camera.projection * camera.view * vertex.pos.to_vec4();

    VaryingOutput {
        varying: vertex.tex_coords,
        position,
    }
}

fn fragment_shader(uniforms: Uniforms, tex_coords: sl::Vec2<f32>) -> sl::Vec4<f32> {
    uniforms.sampler.lookup(tex_coords)
}

// Host code

struct Demo {
    context: Context,
    program: Program<Uniforms, Vertex>,
    camera: UniformBuffer<Camera>,
    vertex_array: VertexArray<Vertex>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, Error> {
        let program = context.create_program(vertex_shader, fragment_shader)?;
        let camera = context.create_uniform_buffer(Camera::default(), BufferUsage::StreamDraw)?;
        let vertex_array = context.create_simple_vertex_array(
            &[
                MyVertex {
                    pos: [0.5f32, 1.0].into(),
                    flag: [false, true].into(),
                }
                .as_std140(),
                MyVertex {
                    pos: [0.0, 0.0].into(),
                    flag: [false, false].into(),
                }
                .as_std140(),
                MyVertex {
                    pos: [1.0, 0.0].into(),
                    flag: [true, true].into(),
                }
                .as_std140(),
            ],
            BufferUsage::StaticDraw,
            (),
        )?;
        let start_time = Instant::now();

        Ok(Self {
            context,
            program,
            camera,
            vertex_array,
            start_time,
        })
    }

    pub fn draw(&self) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.camera.set(MyUniform { time });

        self.context.clear_color([0.1, 0.2, 0.3, 1.0]);
        self.program.draw(
            Uniforms {
                camera: self.camera.binding(),
            },
            self.vertex_array
                .range_binding(0..3, GeometryType::Triangles),
            &DefaultFramebuffer,
            &DrawParams::default(),
        );
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Hello triangle!", 1024, 768)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let context = Context::new(unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    });

    let demo = Demo::new(context).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();
    let mut running = true;

    while running {
        for event in event_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                _ => {}
            }
        }

        demo.draw();
        window.gl_swap_window();
    }
}
