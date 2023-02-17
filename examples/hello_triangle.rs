use std::time::Instant;

use posh::{
    gl::{
        BufferUsage, Context, DefaultFramebuffer, DrawParams, Error, GeometryType, Program,
        UniformBuffer, VertexArray,
    },
    sl::{self, VaryingOutput},
    Block, BlockView, Logical,
};

// Shader interface

#[derive(Clone, Copy, Block)]
struct MyUniform<V: BlockView = Logical> {
    time: V::F32,
}

// Shader code

fn vertex_shader(_: MyUniform, vertex: sl::Vec2<f32>) -> VaryingOutput<sl::Vec2<f32>> {
    let vertex = vertex - sl::vec2(0.5, 0.5);

    VaryingOutput {
        varying: vertex,
        position: sl::vec4(vertex.x, vertex.y, 0.0, 1.0),
    }
}

fn fragment_shader(uniform: MyUniform, varying: sl::Vec2<f32>) -> sl::Vec4<f32> {
    let rg = (varying + uniform.time).cos().pow(sl::vec2(2.0, 2.0));

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

// Host code

struct Demo {
    context: Context,
    program: Program<MyUniform, sl::Vec2<f32>>,
    uniform_buffer: UniformBuffer<MyUniform>,
    vertex_array: VertexArray<sl::Vec2<f32>>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, Error> {
        let program = context.create_program(vertex_shader, fragment_shader)?;
        let uniform_buffer =
            context.create_uniform_buffer(MyUniform { time: 0.0 }, BufferUsage::StreamDraw)?;
        let vertex_array = context.create_simple_vertex_array(
            &[[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()],
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
            self.uniform_buffer.binding(),
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
