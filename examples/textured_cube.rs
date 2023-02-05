use std::time::Instant;

use crevice::std140::AsStd140;
use image::{io::Reader as ImageReader, EncodableLayout, GenericImageView};

use posh::{
    gl::{
        BufferUsage, Context, DefaultFramebuffer, DrawParams, Error, GeometryType, Program,
        RgbaFormat, RgbaImage, Sampler2dParams, Texture2d, UniformBuffer, VertexArray,
    },
    sl::{self, VaryingOutput},
    Block, BlockDomain, Gl, Sl, UniformDomain, UniformInterface,
};

// Shader interface

#[derive(Clone, Copy, Block)]
struct Camera<D: BlockDomain = Sl> {
    projection: D::Mat4,
    view: D::Mat4,
}

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            projection: glam::Mat4::IDENTITY.into(),
            view: glam::Mat4::IDENTITY.into(),
        }
    }
}

#[derive(Clone, Copy, Block)]
struct Vertex<D: BlockDomain = Sl> {
    pos: D::Vec3<f32>,
    tex_coords: D::Vec2<f32>,
}

#[derive(UniformInterface)]
struct Uniforms<D: UniformDomain = Sl> {
    camera: D::Block<Camera>,
    sampler: D::Sampler2d<sl::Vec4<f32>>,
}

// Shader code

fn vertex_shader(uniforms: Uniforms, vertex: Vertex) -> VaryingOutput<sl::Vec2<f32>> {
    let camera = uniforms.camera;
    let position = camera.projection * camera.view * vertex.pos.extend(1.0);

    VaryingOutput {
        varying: vertex.tex_coords,
        position,
    }
}

fn fragment_shader(uniforms: Uniforms, tex_coords: sl::Vec2<f32>) -> sl::Vec4<f32> {
    uniforms.sampler.lookup(tex_coords)
}

// Host code

struct Demo {
    context: Context,
    program: Program<Uniforms, Vertex>,
    camera: UniformBuffer<Camera>,
    vertex_array: VertexArray<Vertex>,
    texture: Texture2d<RgbaFormat>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, Error> {
        let program = context.create_program(vertex_shader, fragment_shader)?;
        let camera = context.create_uniform_buffer(Camera::default(), BufferUsage::StreamDraw)?;
        let vertex_array = context.create_simple_vertex_array(
            &cube_vertices()
                .iter()
                .map(AsStd140::as_std140)
                .collect::<Vec<_>>(),
            BufferUsage::StaticDraw,
            (),
        )?;
        let image = ImageReader::open("examples/resources/smile.png")
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8(); // TODO: anyhow
        let texture = context.create_texture_2d_with_mipmap(RgbaImage::u8_slice(
            image.dimensions(),
            image.as_bytes(),
        ))?;
        let start_time = Instant::now();

        Ok(Self {
            context,
            program,
            camera,
            vertex_array,
            texture,
            start_time,
        })
    }

    pub fn draw(&self) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();

        self.context.clear_color([0.1, 0.2, 0.3, 1.0]);
        self.program.draw(
            Uniforms {
                camera: self.camera.binding(),
                sampler: self.texture.sampler(Sampler2dParams::default()),
            },
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
        .window("Hello texture cube!", 1024, 768)
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

fn cube_vertices() -> Vec<Vertex<Gl>> {
    let tex_coords = [
        [0.0, 0.0].into(),
        [0.0, 1.0].into(),
        [1.0, 1.0].into(),
        [1.0, 0.0].into(),
    ];

    [
        Vertex {
            pos: [0.5, -0.5, -0.5].into(),
            tex_coords: tex_coords[0],
        },
        Vertex {
            pos: [0.5, -0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, 0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, 0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, -0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, 0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, 0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, -0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, 0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, 0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, 0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, 0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, -0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, -0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, -0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, -0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, -0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, 0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, 0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, -0.5, 0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, -0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, -0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [0.5, 0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
        Vertex {
            pos: [-0.5, 0.5, -0.5].into(),
            tex_coords: tex_coords[1],
        },
    ]
    .to_vec()
}
