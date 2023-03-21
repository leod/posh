use std::time::Instant;

use posh::{gl, sl, Block, BlockDom, Sl};

// Shader interface

#[derive(Clone, Copy, Block)]
pub struct State<D: BlockDom = Sl> {
    time: D::F32,
    flip: D::U32,
}

// Shaders

mod scene_pass {
    use posh::sl;

    use super::State;

    pub fn vertex(_: (), input: sl::Vec2) -> sl::VaryingOutput<sl::Vec2> {
        let vertex = input - sl::vec2(0.5, 0.5);

        sl::VaryingOutput {
            output: vertex,
            position: vertex.extend(0.0).extend(1.0),
        }
    }

    pub fn fragment(state: State, input: sl::Vec2) -> sl::Vec4 {
        let rg = (input + state.time).cos().pow(sl::vec2(2.0, 2.0));

        sl::vec4(rg.x, rg.y, 0.5, 1.0)
    }
}

mod present_pass {
    use posh::{sl, Block, BlockDom, Sl, UniformDom};

    use super::State;

    #[derive(Clone, Copy, Block)]
    pub struct Vertex<D: BlockDom = Sl> {
        pub pos: D::Vec2,
        pub tex_coords: D::Vec2,
    }

    #[derive(posh::Uniform)]
    pub struct Uniform<D: UniformDom = Sl> {
        pub state: D::Block<State>,
        pub scene: D::ColorSampler2d<sl::Vec4>,
    }

    pub fn vertex(_: (), vertex: Vertex) -> sl::VaryingOutput<sl::Vec2> {
        sl::VaryingOutput {
            output: vertex.tex_coords,
            position: vertex.pos.extend(0.0).extend(1.0),
        }
    }

    pub fn fragment(uniform: Uniform, tex_coords: sl::Vec2) -> sl::Vec4 {
        let flip = uniform.state.flip;
        let coords = flip.eq(0u32).branch(tex_coords, tex_coords * -1.0);

        uniform.scene.sample(coords)
    }
}

// Host code

struct Demo {
    scene_program: gl::Program<State, sl::Vec2>,
    present_program: gl::Program<present_pass::Uniform, present_pass::Vertex>,

    state: gl::UniformBuffer<State>,
    texture: gl::ColorTexture2d<sl::Vec4>,

    triangle_vertices: gl::VertexBuffer<sl::Vec2>,
    quad_vertices: gl::VertexBuffer<present_pass::Vertex>,
    quad_elements: gl::ElementBuffer,

    start_time: Instant,
}

impl Demo {
    pub fn new(context: gl::Context) -> Result<Self, gl::CreateError> {
        let scene_program = context.create_program(scene_pass::vertex, scene_pass::fragment)?;
        let present_program =
            context.create_program(present_pass::vertex, present_pass::fragment)?;

        let state = context
            .create_uniform_buffer(State { time: 0.0, flip: 0 }, gl::BufferUsage::StreamDraw)?;
        let texture =
            context.create_color_texture_2d(gl::ColorImage::zero_u8(glam::uvec2(1024, 768)))?;

        let triangle_vertices = context.create_vertex_buffer(
            &[[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()],
            gl::BufferUsage::StaticDraw,
        )?;
        let quad_vertices = context.create_vertex_buffer(
            &[
                present_pass::Vertex {
                    pos: [-1.0, -1.0].into(),
                    tex_coords: [0.0, 0.0].into(),
                },
                present_pass::Vertex {
                    pos: [1.0, -1.0].into(),
                    tex_coords: [1.0, 0.0].into(),
                },
                present_pass::Vertex {
                    pos: [1.0, 1.0].into(),
                    tex_coords: [1.0, 1.0].into(),
                },
                present_pass::Vertex {
                    pos: [-1.0, 1.0].into(),
                    tex_coords: [0.0, 1.0].into(),
                },
            ],
            gl::BufferUsage::StaticDraw,
        )?;
        let quad_elements =
            context.create_element_buffer(&[0, 1, 2, 0, 2, 3], gl::BufferUsage::StaticDraw)?;

        let start_time = Instant::now();

        Ok(Self {
            scene_program,
            present_program,
            state,
            texture,
            triangle_vertices,
            quad_vertices,
            quad_elements,
            start_time,
        })
    }

    pub fn draw(&self, flip: u32) -> Result<(), gl::DrawError> {
        self.state.set(State {
            time: Instant::now().duration_since(self.start_time).as_secs_f32(),
            flip,
        });

        self.scene_program.draw(
            self.state.as_binding(),
            gl::VertexStream {
                vertices: self.triangle_vertices.as_binding(),
                elements: gl::Elements::Range(0..3),
                primitive: gl::PrimitiveType::Triangles,
            },
            self.texture.as_color_attachment(),
            gl::DrawParams::default(),
        )?;

        self.present_program.draw(
            present_pass::Uniform {
                state: self.state.as_binding(),
                scene: self
                    .texture
                    .as_color_sampler(gl::Sampler2dParams::default()),
            },
            gl::VertexStream {
                vertices: self.quad_vertices.as_binding(),
                elements: self.quad_elements.as_binding(),
                primitive: gl::PrimitiveType::Triangles,
            },
            gl::DefaultFramebuffer::default(),
            gl::DrawParams::default(),
        )?;

        Ok(())
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
        .window("Press F to flip the triangle (amaze!)", 1024, 768)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let context = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let context = gl::Context::new(context).unwrap();
    let demo = Demo::new(context).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();
    let mut running = true;
    let mut flip = 0;

    while running {
        for event in event_loop.poll_iter() {
            use sdl2::{event::Event::*, keyboard::Keycode};

            match event {
                Quit { .. } => running = false,
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
