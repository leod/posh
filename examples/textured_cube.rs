use std::time::Instant;

use image::{io::Reader as ImageReader, EncodableLayout};

use posh::{
    gl::{
        BufferUsage, Context, DrawParams, ElementBuffer, Error, FramebufferBinding, Image,
        PrimitiveType, Program, Sampler2dParams, Texture2d, UniformBuffer, VertexBuffer,
        VertexStream,
    },
    sl::{self, VaryingOutput},
    Block, BlockFields, GlView, SlView, UniformFields,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

// Shader interface

#[derive(Clone, Copy, Block)]
struct Camera<F: BlockFields = SlView> {
    projection: F::Mat4,
    view: F::Mat4,
}

impl Default for Camera<GlView> {
    fn default() -> Self {
        Self {
            projection: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                WIDTH as f32 / HEIGHT as f32,
                1.0,
                10.0,
            ),
            view: glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -3.0)),
        }
    }
}

#[derive(Clone, Copy, Block)]
struct Vertex<F: BlockFields = SlView> {
    pos: F::Vec3,
    tex_coords: F::Vec2,
}

#[derive(posh::Uniform)]
struct Uniform<F: UniformFields = SlView> {
    camera: F::Block<Camera>,
    time: F::Block<sl::F32>,
}

// Shader code

fn rotate(angle: sl::F32) -> sl::Mat2 {
    sl::mat2(
        sl::vec2(angle.cos(), angle.sin()),
        sl::vec2(angle.sin() * -1.0, angle.cos()),
    )
}

fn zxy(v: sl::Vec3) -> sl::Vec3 {
    sl::vec3(v.z, v.x, v.y)
}

fn vertex_shader(uniforms: Uniform, vertex: Vertex) -> VaryingOutput<sl::Vec2> {
    let camera = uniforms.camera;
    let time = uniforms.time / 3.0;
    let vertex_pos = (rotate(time) * sl::vec2(vertex.pos.x, vertex.pos.y)).extend(vertex.pos.z);
    let position = camera.projection * camera.view * zxy(vertex_pos).extend(1.0);

    VaryingOutput {
        varying: vertex.tex_coords,
        position,
    }
}

// Host code

struct Demo {
    context: Context,
    program: Program<(Uniform, sl::Sampler2d<sl::Vec4>), Vertex>,
    camera: UniformBuffer<Camera>,
    time: UniformBuffer<sl::F32>,
    vertices: VertexBuffer<Vertex>,
    elements: ElementBuffer,
    texture: Texture2d<sl::Vec4>,
    start_time: Instant,
}

impl Demo {
    pub fn new(context: Context) -> Result<Self, Error> {
        let program = context.create_program(vertex_shader, sl::Sampler2d::lookup)?;
        let camera = context.create_uniform_buffer(Camera::default(), BufferUsage::StaticDraw)?;
        let time = context.create_uniform_buffer(0.0, BufferUsage::StreamDraw)?;
        let vertices = context.create_vertex_buffer(&cube_vertices(), BufferUsage::StaticDraw)?;
        let elements = context.create_element_buffer(&cube_elements(), BufferUsage::StaticDraw)?;
        let image = ImageReader::open("examples/resources/smile.png")
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let texture = context.create_texture_2d_with_mipmap(Image::slice_u8(
            image.dimensions().into(),
            image.as_bytes(),
        ))?;
        let start_time = Instant::now();

        Ok(Self {
            context,
            program,
            camera,
            time,
            vertices,
            elements,
            texture,
            start_time,
        })
    }

    pub fn draw(&self) {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.time.set(time);

        self.context.clear_color(glam::vec4(0.1, 0.2, 0.3, 1.0));
        self.program.draw(
            (
                Uniform {
                    camera: self.camera.binding(),
                    time: self.time.binding(),
                },
                self.texture.sampler(Sampler2dParams::default()),
            ),
            VertexStream::Indexed {
                vertices: self.vertices.binding(),
                elements: self.elements.binding(),
                primitive: PrimitiveType::Triangles,
            },
            FramebufferBinding::default(),
            DrawParams::default(),
        );
    }
}

fn cube_vertices() -> Vec<Vertex<GlView>> {
    [
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, 0.5, -0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, 0.5, -0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [0.5, -0.5, 0.5],
        [0.5, -0.5, -0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, 0.5],
        [0.5, 0.5, 0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, 0.5, -0.5],
        [-0.5, 0.5, -0.5],
    ]
    .into_iter()
    .enumerate()
    .map(|(i, pos)| Vertex {
        pos: pos.into(),
        tex_coords: [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]][i % 4].into(),
    })
    .collect()
}

fn cube_elements() -> Vec<u32> {
    (0..6u32)
        .flat_map(|f| [0, 1, 2, 0, 2, 3].map(|j| f * 4 + j))
        .collect()
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Hello textured cube!", WIDTH, HEIGHT)
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
