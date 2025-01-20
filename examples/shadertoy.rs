// Converted from <https://www.shadertoy.com/view/mtyGWy>, created by kishimisu
// on 2023-05-20.

mod utils;

use std::f32::consts::TAU;

use instant::Instant;

use posh::{gl, sl, Block, BlockDom, Gl, Sl, ToSl};

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Globals<D: BlockDom> {
    time: D::F32,
    resolution: D::UVec2,
}

// Shader code

const VERTICES: [glam::Vec4; 6] = [
    glam::vec4(1.0, 1.0, 0.0, 1.0),
    glam::vec4(1.0, -1.0, 0.0, 1.0),
    glam::vec4(-1.0, 1.0, 0.0, 1.0),
    glam::vec4(-1.0, 1.0, 0.0, 1.0),
    glam::vec4(-1.0, -1.0, 0.0, 1.0),
    glam::vec4(1.0, -1.0, 0.0, 1.0),
];

fn vertex_shader(input: sl::VsInput<()>) -> sl::Vec4 {
    VERTICES.to_sl().get(input.vertex_id)
}

fn palette(t: sl::F32) -> sl::Vec3 {
    let a = sl::vec3(0.5, 0.5, 0.5);
    let b = sl::vec3(0.5, 0.5, 0.5);
    let c = sl::vec3(1.0, 1.0, 1.0);
    let d = sl::vec3(0.263, 0.416, 0.557);

    a + b * (TAU * (c * t + d)).cos()
}

fn fragment_shader(Globals { time, resolution }: Globals<Sl>, input: sl::FsInput<()>) -> sl::Vec4 {
    let uv = (input.fragment_coord.xy() * 2.0 - resolution.as_vec2()) / resolution.y.as_f32();
    let len = uv.length();

    (0..4)
        .scan(uv, |uv, i| {
            *uv = (*uv * 1.5).fract() - 0.5;
            Some((i, *uv))
        })
        .map(|(i, uv)| {
            let d = uv.length() * (-len).exp();
            let d = (d * 8.0 + time).sin() / 8.0;
            let d = d.abs();
            let d = (0.01f32 / d).powf(1.2);

            d * palette(len + ((i as f32).to_sl() + time) * 0.4)
        })
        .sum::<sl::Vec3>()
        .extend(1.0)
}

// Host code

struct Demo {
    program: gl::Program<Globals<Sl>, ()>,
    globals: gl::UniformBuffer<Globals<Gl>>,
    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        let globals = Globals {
            time: 0.0,
            resolution: [SCREEN_WIDTH, SCREEN_HEIGHT].into(),
        };

        Ok(Self {
            program: gl.create_program(vertex_shader, fragment_shader)?,
            globals: gl.create_uniform_buffer(globals, StreamDraw)?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&mut self) -> Result<(), gl::DrawError> {
        self.globals.set(Globals {
            time: Instant::now().duration_since(self.start_time).as_secs_f32(),
            resolution: [SCREEN_WIDTH, SCREEN_HEIGHT].into(),
        });

        self.program
            .with_uniforms(self.globals.as_binding())
            .draw(gl::PrimitiveMode::Triangles.as_vertex_spec_with_range(0..VERTICES.len()))?;

        Ok(())
    }
}

// Platform glue

fn main() {
    utils::run_demo("Shadertoy", Demo::new, Demo::draw);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn run() {
    utils::init_wasm().await;

    #[allow(clippy::main_recursion)]
    main();
}
