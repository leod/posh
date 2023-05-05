use std::time::Instant;

use posh::{gl, sl, Block, BlockDom, Gl, Sl, UniformInterface, UniformInterfaceDom};

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

#[derive(UniformInterface)]
pub struct PresentUniforms<D: UniformInterfaceDom> {
    pub state: D::Block<State<Sl>>,
    pub scene: D::ColorSampler2d<sl::Vec4>,
}

// Shaders

mod scene_pass {
    use posh::{sl, Sl};

    use super::State;

    pub fn vertex_stage(_: (), vertex: sl::Vec2) -> sl::VsOut<sl::Vec2> {
        let vertex = vertex - sl::vec2(0.5, 0.5);

        sl::VsOut {
            position: vertex.extend(0.0).extend(1.0),
            varying: vertex,
        }
    }

    pub fn fragment_stage(state: State<Sl>, varying: sl::Vec2) -> sl::Vec4 {
        let rg = (varying + state.time).cos().powf(2.0);

        sl::vec4(rg.x, rg.y, 0.5, 1.0)
    }
}

mod present_pass {
    use posh::{sl, Sl};

    use super::{PresentUniforms, PresentVertex};

    pub fn vertex_stage(_: (), vertex: PresentVertex<Sl>) -> sl::VsOut<sl::Vec2> {
        sl::VsOut {
            position: vertex.pos.extend(0.0).extend(1.0),
            varying: vertex.tex_coords,
        }
    }

    pub fn fragment_stage(uniforms: PresentUniforms<Sl>, tex_coords: sl::Vec2) -> sl::Vec4 {
        let flip = uniforms.state.flip;
        let coords = sl::branch(flip.eq(0u32), tex_coords, -tex_coords);

        uniforms.scene.sample(coords)
    }
}

// Host code

struct Demo {
    scene_program: gl::Program<State<Sl>, sl::Vec2>,
    present_program: gl::Program<PresentUniforms<Sl>, PresentVertex<Sl>>,

    state: gl::UniformBuffer<State<Sl>>,
    texture: gl::ColorTexture2d,

    triangle_vertices: gl::VertexBuffer<sl::Vec2>,
    quad_vertices: gl::VertexBuffer<PresentVertex<Sl>>,
    quad_elements: gl::ElementBuffer,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        let image = gl::ColorImage::rgba_u8_zero([1024, 768]);

        Ok(Self {
            scene_program: gl
                .create_program(scene_pass::vertex_stage, scene_pass::fragment_stage)?,
            present_program: gl
                .create_program(present_pass::vertex_stage, present_pass::fragment_stage)?,
            state: gl.create_uniform_buffer(State { time: 0.0, flip: 0 }, StreamDraw)?,
            texture: gl.create_color_texture_2d(image)?,
            triangle_vertices: gl.create_vertex_buffer(&triangle_vertices(), StaticDraw)?,
            quad_vertices: gl.create_vertex_buffer(&quad_vertices(), StaticDraw)?,
            quad_elements: gl.create_element_buffer(&[0, 1, 2, 0, 2, 3], StaticDraw)?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&self, flip: u32) -> Result<(), gl::DrawError> {
        self.state.set(State {
            time: Instant::now().duration_since(self.start_time).as_secs_f32(),
            flip,
        });

        self.scene_program.draw(
            gl::DrawInputs {
                uniforms: &self.state.as_binding(),
                vertex_spec: &self
                    .triangle_vertices
                    .as_vertex_spec(gl::PrimitiveMode::Triangles),
                settings: &gl::DrawSettings::default(),
            },
            self.texture.as_color_attachment(),
        )?;

        self.present_program.draw(
            gl::DrawInputs {
                uniforms: &PresentUniforms {
                    state: self.state.as_binding(),
                    scene: self
                        .texture
                        .as_color_sampler(gl::Sampler2dSettings::linear()),
                },
                vertex_spec: &self
                    .quad_vertices
                    .as_vertex_spec(gl::PrimitiveMode::Triangles)
                    .with_element_data(self.quad_elements.as_binding()),
                settings: &gl::DrawSettings::default(),
            },
            gl::Framebuffer::default(),
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

// SDL glue

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Press F to flip the triangle (amaze!)", 1024, 768)
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
    let mut flip = 0;

    loop {
        for event in event_loop.poll_iter() {
            use sdl2::{event::Event::*, keyboard::Keycode};

            match event {
                Quit { .. } => return,
                KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => flip = 1,
                KeyUp {
                    keycode: Some(Keycode::F),
                    ..
                } => flip = 0,
                _ => {}
            }
        }

        demo.draw(flip).unwrap();
        window.gl_swap_window();
    }
}
