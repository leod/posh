use std::time::Instant;

use crevice::std140::AsStd140;
use posh::{
    gl::{
        BufferUsage, Context, CreateError, DefaultFramebuffer, DrawParams, GeometryType, Program,
        UniformBuffer, VertexArray,
    },
    sl::{self, VaryingOutput},
    Block, BlockDomain, Sl,
};

#[derive(Clone, Copy, Block)]
struct MyUniform<D: BlockDomain = Sl> {
    time: D::F32,
}

#[derive(Clone, Copy, Block)]
struct MyVertex<D: BlockDomain = Sl> {
    pos: D::Vec2<f32>,
    flag: D::Vec2<bool>,
}

fn vertex_shader<Res>(_: Res, vertex: MyVertex) -> VaryingOutput<sl::Vec2<f32>> {
    let shifted_pos = vertex.pos - 0.5 * vertex.flag.x.branch(1.0, 2.0);
    let shifted_pos = sl::Mat2::identity().x * sl::Vec2::default() + shifted_pos;

    VaryingOutput {
        varying: vertex.pos,
        position: sl::vec4(shifted_pos.x, shifted_pos.y, 0.0, 1.0),
    }
}

fn fragment_shader(uniform: MyUniform, varying: sl::Vec2<f32>) -> sl::Vec4<f32> {
    let rg = (varying + uniform.time).cos().pow(sl::vec2(2.0, 2.0));

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

struct Demo {
    context: Context,
    program: Program<MyUniform, MyVertex>,
    uniform_buffer: UniformBuffer<MyUniform>,
    vertex_array: VertexArray<MyVertex>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, CreateError> {
        let program = context.create_program(vertex_shader, fragment_shader)?;
        let uniform_buffer =
            context.create_uniform_buffer(MyUniform { time: 0.0 }, BufferUsage::StreamDraw)?;
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
            uniform_buffer,
            vertex_array,
            start_time,
        })
    }

    pub fn draw(&self) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.uniform_buffer.set(MyUniform { time });

        self.context.clear_color([0.1, 0.2, 0.3, 1.0]);
        self.program.draw(
            self.uniform_buffer.bind(),
            self.vertex_array
                .stream_range(0..3, GeometryType::Triangles),
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
        .window("Hello triangle!", 1024, 769)
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
