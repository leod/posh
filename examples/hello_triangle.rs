use std::time::Instant;

use posh::{
    gl::{
        BufferUsage, Context, CreateError, DefaultFramebuffer, DrawParams, GeometryType, Program,
        UniformBuffer, VertexArray,
    },
    sl::{self, FragmentInput, FragmentOutput, ToValue, VertexInput, VertexOutput},
    Domain, Sl, Uniform,
};

#[derive(Clone, Copy, ToValue, Uniform)]
struct MyUniform<D: Domain = Sl> {
    time: D::F32,
}

fn vertex_shader(_: MyUniform, input: VertexInput<sl::Vec2<f32>>) -> VertexOutput<sl::Vec2<f32>> {
    let shifted_pos = input.vertex - 0.5;

    VertexOutput::new(
        sl::vec4(shifted_pos.x, shifted_pos.y, 0.0, 1.0),
        input.vertex,
    )
}

fn fragment_shader(
    uniform: MyUniform,
    input: FragmentInput<sl::Vec2<f32>>,
) -> FragmentOutput<sl::Vec4<f32>> {
    let rg = (input.varying + uniform.time).cos().pow(sl::vec2(2.0, 2.0));

    FragmentOutput::new(sl::vec4(rg.x, rg.y, 0.5, 1.0))
}

struct Demo {
    context: Context,
    program: Program<MyUniform, sl::Vec2<f32>, sl::Vec4<f32>>,
    uniform_buffer: UniformBuffer<MyUniform>,
    vertex_array: VertexArray<sl::Vec2<f32>>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, CreateError> {
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
