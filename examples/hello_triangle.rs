use std::time::Instant;

use posh::{gl, sl, Block, BlockDom, Sl};

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Globals<D: BlockDom = Sl> {
    time: D::F32,
}

// Shader code

fn vertex_shader(_: (), vertex: sl::Vec2) -> sl::VsOut<sl::Vec2> {
    let pos = vertex - sl::vec2(0.5, 0.5);

    sl::VsOut {
        position: sl::vec4(pos.x, pos.y, 0.0, 1.0),
        varying: pos,
    }
}

fn fragment_shader(uniform: Globals, varying: sl::Vec2) -> sl::Vec4 {
    let rg = (varying + uniform.time).cos().powf(2.0);

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

// Host code

struct Demo {
    program: gl::Program<Globals, sl::Vec2>,

    globals: gl::UniformBuffer<Globals>,
    vertices: gl::VertexBuffer<sl::Vec2>,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        let globals = Globals { time: 0.0 };
        let vertices = vec![[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()];

        Ok(Self {
            program: gl.create_program(vertex_shader, fragment_shader)?,
            globals: gl.create_uniform_buffer(globals, StreamDraw)?,
            vertices: gl.create_vertex_buffer(&vertices, StaticDraw)?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        self.globals.set(Globals {
            time: Instant::now().duration_since(self.start_time).as_secs_f32(),
        });

        self.program.draw(
            gl::Input {
                uniform: &self.globals.as_binding(),
                vertex: &self.vertices.as_vertex_spec(gl::Mode::Triangles),
                settings: &gl::Settings::default().with_clear_color([0.1, 0.2, 0.3, 1.0]),
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
        .window("Hello triangle!", 1024, 768)
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
