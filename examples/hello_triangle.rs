use std::time::Instant;

use posh::{
    gl::{Context, CreateError, Program},
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
        sl::vec4(shifted_pos.x - 0.5, shifted_pos.y - 0.5, 0.0, 1.0),
        input.vertex,
    )
}

fn fragment_shader(
    _: MyUniform,
    input: FragmentInput<sl::Vec2<f32>>,
) -> FragmentOutput<sl::Vec4<f32>> {
    let rg = input.varying;

    FragmentOutput::new(sl::vec4(rg.x, rg.y, 0.5, 1.0))
}

struct Demo {
    program: Program<MyUniform, sl::Vec2<f32>, sl::Vec4<f32>>,
}

impl Demo {
    pub fn new(context: &Context) -> Result<Self, CreateError> {
        let program = context.create_program(vertex_shader, fragment_shader)?;

        Ok(Self { program })
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

    let demo = Demo::new(&context).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();

    let start_time = Instant::now();
    let mut running = true;

    while running {
        for event in event_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                _ => {}
            }
        }

        let time = Instant::now().duration_since(start_time).as_secs_f32();

        window.gl_swap_window();
    }
}
