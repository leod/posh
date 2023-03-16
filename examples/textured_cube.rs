use std::time::Instant;

use image::{io::Reader as ImageReader, EncodableLayout};

use posh::{gl, sl, Block, BlockDom, Gl, Sl, UniformDom};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

// Shader interface

#[derive(Clone, Copy, Block)]
struct Camera<D: BlockDom = Sl> {
    projection: D::Mat4,
    view: D::Mat4,
}

impl Default for Camera<Gl> {
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
struct Vertex<D: BlockDom = Sl> {
    pos: D::Vec3,
    tex_coords: D::Vec2,
}

#[derive(posh::Uniform)]
struct Uniform<D: UniformDom = Sl> {
    camera: D::Block<Camera>,
    time: D::Block<sl::F32>,
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

fn vertex_shader(uniforms: Uniform, vertex: Vertex) -> sl::VaryingOutput<sl::Vec2> {
    let camera = uniforms.camera;
    let time = uniforms.time / 3.0;

    let vertex_pos = (rotate(time) * sl::vec2(vertex.pos.x, vertex.pos.y)).extend(vertex.pos.z);
    let position = camera.projection * camera.view * zxy(vertex_pos).extend(1.0);

    sl::VaryingOutput {
        varying: vertex.tex_coords,
        position,
    }
}

// Host code

struct Demo {
    program: gl::Program<(Uniform, sl::Sampler2d<sl::Vec4>), Vertex>,

    camera: gl::UniformBuffer<Camera>,
    time: gl::UniformBuffer<sl::F32>,
    vertices: gl::VertexBuffer<Vertex>,
    elements: gl::ElementBuffer,
    texture: gl::Texture2d<sl::Vec4>,

    start_time: Instant,
}

impl Demo {
    pub fn new(context: gl::Context) -> Result<Self, gl::CreateError> {
        let program = context.create_program(vertex_shader, sl::Sampler2d::lookup)?;

        let camera =
            context.create_uniform_buffer(Camera::default(), gl::BufferUsage::StaticDraw)?;
        let time = context.create_uniform_buffer(0.0, gl::BufferUsage::StreamDraw)?;
        let vertices =
            context.create_vertex_buffer(&cube_vertices(), gl::BufferUsage::StaticDraw)?;
        let elements =
            context.create_element_buffer(&cube_elements(), gl::BufferUsage::StaticDraw)?;
        let image = ImageReader::open("examples/resources/smile.png")
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let texture = context.create_texture_2d_with_mipmap(gl::Image::slice_u8(
            image.dimensions().into(),
            image.as_bytes(),
        ))?;

        let start_time = Instant::now();

        Ok(Self {
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

        let uniform = Uniform {
            camera: self.camera.binding(),
            time: self.time.binding(),
        };
        let sampler = self.texture.sampler(gl::Sampler2dParams::default());

        self.program
            .draw(
                (uniform, sampler),
                gl::VertexStream {
                    vertices: self.vertices.binding(),
                    elements: self.elements.binding(),
                    primitive: gl::PrimitiveType::Triangles,
                },
                gl::DefaultFramebuffer::default(),
                gl::DrawParams::default()
                    .with_clear_color(glam::vec4(0.1, 0.2, 0.3, 1.0))
                    .with_clear_depth(1.0)
                    .with_depth_compare(gl::CompareFunction::Less),
            )
            .unwrap();
    }
}

// Mesh data

fn cube_vertices() -> Vec<Vertex<Gl>> {
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

// Main loop

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Hello textured cube!", WIDTH, HEIGHT)
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

    while running {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            if matches!(event, Quit { .. }) {
                running = false;
            }
        }

        demo.draw();
        window.gl_swap_window();
    }
}
