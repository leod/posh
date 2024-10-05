mod utils;

use instant::Instant;

use posh::{gl, sl, Block, BlockDom, Gl, Sl};

// Shader interface

#[derive(Clone, Copy, Default, Block)]
#[repr(C)]
struct Globals<D: BlockDom> {
    time: D::F32,
    triangle_size: D::Vec2,
}

// Shader code

fn vertex_shader(globals: Globals<Sl>, vertex: sl::Vec2) -> sl::VsOutput<sl::Vec2> {
    let position = sl::Vec2::from_angle(globals.time).rotate(vertex * globals.triangle_size);

    sl::VsOutput {
        clip_pos: sl::vec4(position.x, position.y, 0.0, 1.0),
        interp: vertex,
    }
}

fn fragment_shader(globals: Globals<Sl>, interp: sl::Vec2) -> sl::Vec4 {
    let rg = (interp + globals.time).cos().powf(2.0);

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

// Host code

struct Demo {
    program: gl::Program<Globals<Sl>, sl::Vec2>,

    globals: gl::UniformBuffer<Globals<Gl>>,
    vertices: gl::VertexBuffer<gl::Vec2>,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        let vertices = vec![
            [0.0f32, 1.0].into(),
            [-0.5, -0.5].into(),
            [0.5, -0.5].into(),
        ];

        Ok(Self {
            program: gl.create_program(vertex_shader, fragment_shader)?,
            globals: gl.create_uniform_buffer(Default::default(), StreamDraw)?,
            vertices: gl.create_vertex_buffer(&vertices, StaticDraw)?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&mut self) -> Result<(), gl::DrawError> {
        self.globals.set(Globals {
            time: Instant::now().duration_since(self.start_time).as_secs_f32(),
            triangle_size: [1.0, 1.0].into(),
        });

        self.program
            .with_uniforms(self.globals.as_binding())
            .with_params(gl::DrawParams::new().with_clear_color([0.1, 0.2, 0.3, 1.0]))
            .draw(self.vertices.as_vertex_spec(gl::PrimitiveMode::Triangles))?;

        Ok(())
    }
}

// Platform glue

fn main() {
    utils::run_demo("Hello, Triangle!", Demo::new, Demo::draw);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn run() {
    utils::init_wasm().await;

    #[allow(clippy::main_recursion)]
    main();
}
