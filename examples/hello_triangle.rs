use std::time::Instant;

use posh::{
    gl::{
        BufferUsage, Context, DrawParams, Error, FramebufferBinding, PrimitiveType, Program,
        UniformBuffer, VertexBuffer, VertexStream,
    },
    sl::{self, VaryingOutput},
    Block, BlockFields, SlView,
};

// Shader interface

#[derive(Clone, Copy, Block)]
struct Globals<F: BlockFields = SlView> {
    time: F::F32,
}

// Shader code

fn vertex_shader(_: (), vertex: sl::Vec2) -> VaryingOutput<sl::Vec2> {
    let vertex = vertex - sl::vec2(0.5, 0.5);

    VaryingOutput {
        varying: vertex,
        position: sl::vec4(vertex.x, vertex.y, 0.0, 1.0),
    }
}

fn fragment_shader(uniform: Globals, varying: sl::Vec2) -> sl::Vec4 {
    let rg = (varying + uniform.time).cos().pow(sl::vec2(2.0, 2.0));

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

// Host code

struct Demo {
    context: Context,
    program: Program<Globals, sl::Vec2>,
    globals: UniformBuffer<Globals>,
    vertices: VertexBuffer<sl::Vec2>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, Error> {
        let program = context.create_program(vertex_shader, fragment_shader)?;
        let globals =
            context.create_uniform_buffer(Globals { time: 0.0 }, BufferUsage::StreamDraw)?;
        let vertices = context.create_vertex_buffer(
            &[[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()],
            BufferUsage::StaticDraw,
        )?;
        let start_time = Instant::now();

        Ok(Self {
            context,
            program,
            globals,
            vertices,
            start_time,
        })
    }

    pub fn draw(&self) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.globals.set(Globals { time });

        self.context.clear_color([0.1, 0.2, 0.3, 1.0]);
        self.program.draw(
            self.globals.binding(),
            VertexStream::Unindexed {
                vertices: self.vertices.binding(),
                range: 0..3,
                primitive: PrimitiveType::Triangles,
            },
            FramebufferBinding::default(),
            DrawParams::default(),
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
    })
    .unwrap();

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
