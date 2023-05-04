use std::time::Instant;

use image::{io::Reader as ImageReader, EncodableLayout};

use posh::{gl, sl, Block, BlockDom, Gl, Sl, UniformBindingsDom};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Camera<D: BlockDom = Sl> {
    world_to_view: D::Mat4,
    view_to_screen: D::Mat4,
}

#[derive(Clone, Copy, Block)]
#[repr(C)]
struct Vertex<D: BlockDom = Sl> {
    pos: D::Vec3,
    tex_coords: D::Vec2,
}

#[derive(posh::UniformBindings)]
struct UniformBindings<D: UniformBindingsDom = Sl> {
    camera: D::Block<Camera>,
    time: D::Block<sl::F32>,
}

// Shader code

fn zxy(v: sl::Vec3) -> sl::Vec3 {
    sl::vec3(v.z, v.x, v.y)
}

fn vertex_shader(uniforms: UniformBindings, vertex: Vertex) -> sl::VsOut<sl::Vec2> {
    let camera = uniforms.camera;

    let vertex_pos = vertex
        .pos
        .xy()
        .rotate(sl::Vec2::from_angle(uniforms.time))
        .extend(vertex.pos.z);
    let position = camera.view_to_screen * camera.world_to_view * zxy(vertex_pos).extend(1.0);

    sl::VsOut {
        varying: vertex.tex_coords,
        position,
    }
}

// Host code

struct Demo {
    program: gl::Program<(UniformBindings, sl::ColorSampler2d), Vertex>,

    camera: gl::UniformBuffer<Camera>,
    time: gl::UniformBuffer<sl::F32>,
    texture: gl::ColorTexture2d<sl::Vec4>,

    vertices: gl::VertexBuffer<Vertex>,
    elements: gl::ElementBuffer,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        let image = ImageReader::open("examples/resources/smile.png")
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let image = gl::ColorImage::rgba_u8_slice(
            [image.dimensions().0, image.dimensions().1],
            image.as_bytes(),
        );

        Ok(Self {
            program: gl.create_program(vertex_shader, sl::ColorSampler2d::sample)?,
            camera: gl.create_uniform_buffer(Camera::default(), StaticDraw)?,
            time: gl.create_uniform_buffer(0.0, StreamDraw)?,
            texture: gl.create_color_texture_2d_with_mipmap(image)?,
            vertices: gl.create_vertex_buffer(&cube_vertices(), StaticDraw)?,
            elements: gl.create_element_buffer(&cube_elements(), StaticDraw)?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.time.set(time);

        self.program.draw(
            gl::Input {
                uniform: &(
                    UniformBindings {
                        camera: self.camera.as_binding(),
                        time: self.time.as_binding(),
                    },
                    self.texture
                        .as_color_sampler(gl::Sampler2dSettings::linear()),
                ),
                vertex: &self
                    .vertices
                    .as_vertex_spec(gl::Mode::Triangles)
                    .with_element_data(self.elements.as_binding()),
                settings: &gl::Settings::default()
                    .with_clear_color([0.1, 0.2, 0.3, 1.0])
                    .with_clear_depth(1.0)
                    .with_depth_test(gl::Comparison::Less),
            },
            gl::Framebuffer::default(),
        )
    }
}

// Scene data

impl Default for Camera<Gl> {
    fn default() -> Self {
        Self {
            world_to_view: glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -3.0)).into(),
            view_to_screen: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                WIDTH as f32 / HEIGHT as f32,
                1.0,
                10.0,
            )
            .into(),
        }
    }
}

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
        .flat_map(|face| [0, 1, 2, 0, 2, 3].map(|i| face * 4 + i))
        .collect()
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
