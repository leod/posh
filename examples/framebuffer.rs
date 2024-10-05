mod utils;

use instant::Instant;

use posh::{gl, sl, Block, BlockDom, Gl, Sl, Uniform, UniformDom};

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct State<D: BlockDom> {
    time: D::F32,
    flip: D::U32,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct PresentVertex<D: BlockDom> {
    pub pos: D::Vec2,
    pub tex_coords: D::Vec2,
}

#[derive(Uniform)]
pub struct PresentUniforms<D: UniformDom> {
    pub state: D::Block<State<Sl>>,
    pub scene: D::ColorSampler2d<sl::Vec4>,
}

// Shaders

mod scene_pass {
    use posh::{sl, Sl};

    use super::State;

    pub fn vertex_shader(vertex: sl::Vec2) -> sl::VsOutput<sl::Vec2> {
        let vertex = vertex - sl::vec2(0.5, 0.5);

        sl::VsOutput {
            clip_position: vertex.extend(0.0).extend(1.0),
            interpolant: vertex,
        }
    }

    pub fn fragment_shader(state: State<Sl>, interpolant: sl::Vec2) -> sl::Vec4 {
        let rg = (interpolant + state.time).cos().powf(2.0);

        sl::vec4(rg.x, rg.y, 0.5, 1.0)
    }
}

mod present_pass {
    use posh::{sl, Sl};

    use super::{PresentUniforms, PresentVertex};

    pub fn vertex_shader(vertex: PresentVertex<Sl>) -> sl::VsOutput<sl::Vec2> {
        sl::VsOutput {
            clip_position: vertex.pos.extend(0.0).extend(1.0),
            interpolant: vertex.tex_coords,
        }
    }

    fn dither(p: sl::Vec2, alpha: sl::F32) -> sl::Bool {
        let thresh = sl::mat4(
            sl::vec4(1., 13., 4., 16.),
            sl::vec4(9., 5., 12., 8.),
            sl::vec4(3., 15., 2., 14.),
            sl::vec4(11., 7., 10., 6.),
        ) / 17.0;

        thresh.get(p.x.as_u32() % 4).get(p.y.as_u32() % 4).ge(alpha)
    }

    pub fn fragment_shader(
        uniforms: PresentUniforms<Sl>,
        input: sl::FsInput<sl::Vec2>,
    ) -> sl::Vec4 {
        let flip = uniforms.state.flip;
        let dithered_coords = dither(
            input.fragment_coord.xy(),
            (uniforms.state.time * 0.3).cos().powf(2.0),
        )
        .then_discard(input)
        .otherwise(input.interpolant);

        let coords = flip
            .eq(1u32)
            .then(dithered_coords)
            .otherwise(input.interpolant);

        uniforms.scene.sample(coords)
    }
}

// Host code

struct Demo {
    scene_program: gl::Program<State<Sl>, sl::Vec2>,
    present_program: gl::Program<PresentUniforms<Sl>, PresentVertex<Sl>>,

    state: gl::UniformBuffer<State<Gl>>,
    texture: gl::ColorTexture2d,

    triangle_vertices: gl::VertexBuffer<gl::Vec2>,
    quad_vertices: gl::VertexBuffer<PresentVertex<Gl>>,
    quad_elements: gl::ElementBuffer,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        let image = gl::ColorImage::rgba_u8_zero([1024, 768]);

        Ok(Self {
            scene_program: gl
                .create_program(scene_pass::vertex_shader, scene_pass::fragment_shader)?,
            present_program: gl
                .create_program(present_pass::vertex_shader, present_pass::fragment_shader)?,
            state: gl.create_uniform_buffer(State { time: 0.0, flip: 0 }, StreamDraw)?,
            texture: gl.create_color_texture_2d(image)?,
            triangle_vertices: gl.create_vertex_buffer(&triangle_vertices(), StaticDraw)?,
            quad_vertices: gl.create_vertex_buffer(&quad_vertices(), StaticDraw)?,
            quad_elements: gl.create_element_buffer(&[0, 1, 2, 0, 2, 3], StaticDraw)?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&mut self) -> Result<(), gl::DrawError> {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        let flip = ((time / 5.0) as usize % 2) as u32;

        self.state.set(State { time, flip });

        self.scene_program
            .with_uniforms(self.state.as_binding())
            .with_framebuffer(self.texture.as_color_attachment())
            .draw(
                self.triangle_vertices
                    .as_vertex_spec(gl::PrimitiveMode::Triangles),
            )?;

        self.present_program
            .with_uniforms(PresentUniforms {
                state: self.state.as_binding(),
                scene: self.texture.as_color_sampler(gl::Sampler2dParams::linear()),
            })
            .draw(
                self.quad_vertices
                    .as_vertex_spec(gl::PrimitiveMode::Triangles)
                    .with_element_data(self.quad_elements.as_binding()),
            )?;

        Ok(())
    }
}

// Scene data

fn triangle_vertices() -> Vec<gl::Vec2> {
    vec![[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()]
}

fn quad_vertices() -> Vec<PresentVertex<Gl>> {
    vec![
        PresentVertex {
            pos: [-1.0, -1.0].into(),
            tex_coords: [0.0, 0.0].into(),
        },
        PresentVertex {
            pos: [1.0, -1.0].into(),
            tex_coords: [1.0, 0.0].into(),
        },
        PresentVertex {
            pos: [1.0, 1.0].into(),
            tex_coords: [1.0, 1.0].into(),
        },
        PresentVertex {
            pos: [-1.0, 1.0].into(),
            tex_coords: [0.0, 1.0].into(),
        },
    ]
}

// Platform glue

fn main() {
    utils::run_demo(
        "Framebuffer and dithering (toggling every 5s)",
        Demo::new,
        Demo::draw,
    );
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn run() {
    utils::init_wasm().await;

    #[allow(clippy::main_recursion)]
    main();
}
