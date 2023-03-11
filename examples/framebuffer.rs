use std::time::Instant;

use posh::{
    gl::{
        BufferUsage, Context, DrawParams, ElementBuffer, Error, Framebuffer, FramebufferBinding,
        Image, PrimitiveType, Program, Sampler2dParams, UniformBuffer, VertexBuffer, VertexStream,
    },
    sl::{self, VaryingOutput},
    Block, BlockFields, SlView, Uniform, UniformFields,
};

// Shader interface

#[derive(Clone, Copy, Block)]
struct Globals<F: BlockFields = SlView> {
    time: F::F32,
    flip: F::U32,
}

#[derive(Clone, Copy, Block)]
struct Vertex<F: BlockFields = SlView> {
    pos: F::Vec2,
    tex_coords: F::Vec2,
}

#[derive(Uniform)]
struct PostUniforms<F: UniformFields = SlView> {
    globals: F::Block<Globals>,
    scene: F::Sampler2d<sl::Vec4>,
}

// Shader code

fn scene_vertex(_: (), vertex: sl::Vec2) -> VaryingOutput<sl::Vec2> {
    let vertex = vertex - sl::vec2(0.5, 0.5);

    VaryingOutput {
        varying: vertex,
        position: vertex.extend(0.0).extend(1.0),
    }
}

fn scene_fragment(uniform: Globals, varying: sl::Vec2) -> sl::Vec4 {
    let rg = (varying + uniform.time).cos().pow(sl::vec2(2.0, 2.0));

    sl::vec4(rg.x, rg.y, 0.5, 1.0)
}

fn post_vertex(_: (), vertex: Vertex) -> VaryingOutput<sl::Vec2> {
    VaryingOutput {
        varying: vertex.tex_coords,
        position: vertex.pos.extend(0.0).extend(1.0),
    }
}

fn post_fragment(uniform: PostUniforms, tex_coords: sl::Vec2) -> sl::Vec4 {
    let flip = uniform.globals.flip;
    let coords = flip.eq(0u32).branch(tex_coords, tex_coords * -1.0);

    uniform.scene.lookup(coords)
}

// Host code

struct Demo {
    context: Context,
    scene_program: Program<Globals, sl::Vec2>,
    post_program: Program<PostUniforms, Vertex>,
    globals: UniformBuffer<Globals>,
    triangle_vertices: VertexBuffer<sl::Vec2>,
    quad_vertices: VertexBuffer<Vertex>,
    quad_elements: ElementBuffer,
    framebuffer: Framebuffer<sl::Vec4>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, Error> {
        let scene_program = context.create_program(scene_vertex, scene_fragment)?;
        let post_program = context.create_program(post_vertex, post_fragment)?;
        let globals = context
            .create_uniform_buffer(Globals { time: 0.0, flip: 0 }, BufferUsage::StreamDraw)?;
        let triangle_vertices = context.create_vertex_buffer(
            &[[0.5f32, 1.0].into(), [0.0, 0.0].into(), [1.0, 0.0].into()],
            BufferUsage::StaticDraw,
        )?;
        let quad_vertices = context.create_vertex_buffer(
            &[
                Vertex {
                    pos: [-1.0, -1.0].into(),
                    tex_coords: [0.0, 0.0].into(),
                },
                Vertex {
                    pos: [1.0, -1.0].into(),
                    tex_coords: [1.0, 0.0].into(),
                },
                Vertex {
                    pos: [1.0, 1.0].into(),
                    tex_coords: [1.0, 1.0].into(),
                },
                Vertex {
                    pos: [-1.0, 1.0].into(),
                    tex_coords: [0.0, 1.0].into(),
                },
            ],
            BufferUsage::StaticDraw,
        )?;
        let quad_elements =
            context.create_element_buffer(&[0, 1, 2, 0, 2, 3], BufferUsage::StaticDraw)?;
        let texture = context.create_texture_2d(Image::zeroed_u8(glam::uvec2(1024, 768)))?;
        let framebuffer = context.create_framebuffer(texture.attachment())?;
        let start_time = Instant::now();

        Ok(Self {
            context,
            scene_program,
            post_program,
            globals,
            triangle_vertices,
            quad_vertices,
            quad_elements,
            framebuffer,
            start_time,
        })
    }

    pub fn draw(&self, flip: u32) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.globals.set(Globals { time, flip });

        self.context.clear_color(glam::vec4(0.1, 0.2, 0.3, 1.0));
        self.scene_program.draw(
            self.globals.binding(),
            VertexStream::Unindexed {
                vertices: self.triangle_vertices.binding(),
                range: 0..3,
                primitive: PrimitiveType::Triangles,
            },
            self.framebuffer.binding(),
            DrawParams::default(),
        );
        self.post_program.draw(
            PostUniforms {
                globals: self.globals.binding(),
                scene: self.framebuffer.view().sampler(Sampler2dParams::default()),
            },
            VertexStream::Indexed {
                vertices: self.quad_vertices.binding(),
                elements: self.quad_elements.binding(),
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
    let mut flip = 0;

    while running {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;
            use sdl2::keyboard::Keycode;

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

        demo.draw(flip);
        window.gl_swap_window();
    }
}
