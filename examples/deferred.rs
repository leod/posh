use std::time::Instant;

use posh::{
    gl, sl, Block, BlockDom, FsInterface, FsInterfaceDom, Gl, Sl, UniformInterface,
    UniformInterfaceDom,
};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

// Shader interface

#[derive(Clone, Copy, Block)]
#[repr(C)]
pub struct Globals<D: BlockDom> {
    world_to_view: D::Mat4,
    view_to_screen: D::Mat4,
    time: D::F32,
}

#[derive(Clone, Copy, UniformInterface)]
pub struct SceneSamplers<D: UniformInterfaceDom> {
    albedo: D::ColorSampler2d<sl::Vec3>,
    world_normal: D::ColorSampler2d<sl::Vec3>,
    world_pos: D::ColorSampler2d<sl::Vec3>,
}

#[derive(Clone, Copy, FsInterface)]
pub struct SceneAttachments<D: FsInterfaceDom> {
    albedo: D::ColorAttachment<sl::Vec3>,
    world_normal: D::ColorAttachment<sl::Vec3>,
    world_pos: D::ColorAttachment<sl::Vec3>,
}

impl SceneAttachments<Gl> {
    fn as_scene_samplers(&self) -> SceneSamplers<Gl> {
        let settings = gl::Sampler2dSettings::linear();

        SceneSamplers {
            albedo: self.albedo.as_color_sampler(settings),
            world_normal: self.world_normal.as_color_sampler(settings),
            world_pos: self.world_pos.as_color_sampler(settings),
        }
    }
}

// Shaders

mod scene_pass {
    use posh::{
        sl::{self, ToSl},
        Sl,
    };

    use crate::SceneAttachments;

    use super::Globals;

    const CUBE_POSITIONS: [glam::Vec3; 24] = [
        glam::vec3(0.5, -0.5, -0.5),
        glam::vec3(0.5, -0.5, 0.5),
        glam::vec3(0.5, 0.5, 0.5),
        glam::vec3(0.5, 0.5, -0.5),
        glam::vec3(-0.5, -0.5, -0.5),
        glam::vec3(-0.5, 0.5, -0.5),
        glam::vec3(-0.5, 0.5, 0.5),
        glam::vec3(-0.5, -0.5, 0.5),
        glam::vec3(-0.5, 0.5, -0.5),
        glam::vec3(0.5, 0.5, -0.5),
        glam::vec3(0.5, 0.5, 0.5),
        glam::vec3(-0.5, 0.5, 0.5),
        glam::vec3(-0.5, -0.5, -0.5),
        glam::vec3(-0.5, -0.5, 0.5),
        glam::vec3(0.5, -0.5, 0.5),
        glam::vec3(0.5, -0.5, -0.5),
        glam::vec3(-0.5, -0.5, 0.5),
        glam::vec3(-0.5, 0.5, 0.5),
        glam::vec3(0.5, 0.5, 0.5),
        glam::vec3(0.5, -0.5, 0.5),
        glam::vec3(-0.5, -0.5, -0.5),
        glam::vec3(0.5, -0.5, -0.5),
        glam::vec3(0.5, 0.5, -0.5),
        glam::vec3(-0.5, 0.5, -0.5),
    ];

    const CUBE_NORMALS: [glam::Vec3; 6] = [
        glam::vec3(1.0, 0.0, 0.0),
        glam::vec3(-1.0, 0.0, 0.0),
        glam::vec3(0.0, 1.0, 0.0),
        glam::vec3(0.0, -1.0, 0.0),
        glam::vec3(0.0, 0.0, 1.0),
        glam::vec3(0.0, 0.0, -1.0),
    ];

    const CUBE_ELEMENTS: [u32; 6] = [0, 1, 2, 0, 2, 3];

    fn zxy(v: sl::Vec3) -> sl::Vec3 {
        sl::vec3(v.z, v.x, v.y)
    }

    pub fn vertex_stage(
        globals: Globals<Sl>,
        input: sl::VsInput<()>,
    ) -> sl::VsOutput<SceneAttachments<Sl>> {
        let vertex_id = input.vertex_id / 6 * 4 + CUBE_ELEMENTS.to_sl().get(input.vertex_id % 6);

        let object_pos = CUBE_POSITIONS
            .to_sl()
            .get(vertex_id % CUBE_POSITIONS.len() as u32);
        let world_pos = object_pos
            .xy()
            .rotate(sl::Vec2::from_angle(globals.time))
            .extend(object_pos.z);
        let screen_pos =
            globals.view_to_screen * globals.world_to_view * zxy(world_pos).extend(1.0);

        // TODO: Fix world normal calculation.
        let world_normal = CUBE_NORMALS.to_sl().get((vertex_id / 4) % 6);

        sl::VsOutput {
            clip_position: screen_pos,
            interpolant: SceneAttachments {
                albedo: sl::vec3(1.0, 0.0, 0.0),
                world_pos,
                world_normal,
            },
        }
    }

    pub fn fragment_stage(_: (), interpolant: SceneAttachments<Sl>) -> SceneAttachments<Sl> {
        interpolant
    }
}

mod present_pass {
    use posh::{
        sl::{self, ToSl},
        Sl,
    };

    use crate::SceneSamplers;

    pub const SQUARE_POSITIONS: [glam::Vec2; 6] = [
        glam::vec2(1., 1.),
        glam::vec2(1., -1.),
        glam::vec2(-1., 1.),
        glam::vec2(-1., 1.),
        glam::vec2(-1., -1.),
        glam::vec2(1., -1.),
    ];

    pub fn vertex_stage(_: (), input: sl::VsInput<()>) -> sl::VsOutput<sl::Vec2> {
        let position = SQUARE_POSITIONS.to_sl().get(input.vertex_id);

        sl::VsOutput {
            clip_position: position.extend(0.0).extend(1.0),
            interpolant: (position + 1.0) / 2.0,
        }
    }

    pub fn fragment_stage(samplers: SceneSamplers<Sl>, uv: sl::Vec2) -> sl::Vec4 {
        let albedo = samplers.albedo.sample(sl::vec2(uv.x * 3.0, uv.y));
        let world_normal = samplers
            .world_normal
            .sample(sl::vec2((uv.x - 2.0) * 3.0, uv.y));
        let world_pos = samplers
            .world_pos
            .sample(sl::vec2((uv.x - 1.0) * 3.0, uv.y));

        sl::branches(
            [
                (uv.x.lt(1.0 / 3.0), albedo),
                (uv.x.lt(2.0 / 3.0), world_normal),
            ],
            world_pos,
        )
        .extend(1.0)
    }
}

// Host code

struct Demo {
    scene_program: gl::Program<Globals<Sl>, (), SceneAttachments<Sl>>,
    present_program: gl::Program<SceneSamplers<Sl>, ()>,

    globals: gl::UniformBuffer<Globals<Gl>>,
    scene_attachments: SceneAttachments<Gl>,
    depth_texture: gl::DepthTexture2d,

    start_time: Instant,
}

impl Demo {
    pub fn new(gl: gl::Context) -> Result<Self, gl::CreateError> {
        use gl::BufferUsage::*;

        // TODO: This example looks kinda broken since it does not use
        // floating-point textures. See also:
        // <https://github.com/leod/posh/issues/131>.

        let scene_texture =
            || gl.create_color_texture_2d(gl::ColorImage::rgb_u8_zero([WIDTH, HEIGHT]));

        let scene_attachments = SceneAttachments {
            albedo: scene_texture()?.as_color_attachment(),
            world_normal: scene_texture()?.as_color_attachment(),
            world_pos: scene_texture()?.as_color_attachment(),
        };

        Ok(Self {
            scene_program: gl
                .create_program(scene_pass::vertex_stage, scene_pass::fragment_stage)?,
            present_program: gl
                .create_program(present_pass::vertex_stage, present_pass::fragment_stage)?,
            globals: gl.create_uniform_buffer(Globals::new(0.0), StreamDraw)?,
            scene_attachments,
            depth_texture: gl.create_depth_texture_2d(gl::DepthImage::f32_zero([WIDTH, HEIGHT]))?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&self) -> Result<(), gl::DrawError> {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.globals.set(Globals::new(time));

        self.scene_program.draw(
            gl::DrawInputs {
                uniforms: &self.globals.as_binding(),
                vertex_spec: &gl::VertexSpec::new(gl::PrimitiveMode::Triangles)
                    .with_vertex_range(0..36),
                settings: &gl::DrawSettings::default()
                    .with_clear_color([0.0; 4])
                    .with_clear_depth(1.0)
                    .with_depth_test(gl::Comparison::Less),
            },
            gl::Framebuffer::color_and_depth(
                &self.scene_attachments,
                &self.depth_texture.as_depth_attachment(),
            ),
        )?;

        self.present_program.draw(
            gl::DrawInputs {
                uniforms: &self.scene_attachments.as_scene_samplers(),
                vertex_spec: &gl::VertexSpec::new(gl::PrimitiveMode::Triangles)
                    .with_vertex_range(0..6),
                settings: &gl::DrawSettings::default(),
            },
            gl::Framebuffer::default(),
        )?;

        Ok(())
    }
}

// Scene data

impl Globals<Gl> {
    fn new(time: f32) -> Self {
        Self {
            world_to_view: glam::Mat4::from_translation(glam::Vec3::new(0.0, 0.0, -3.0)).into(),
            view_to_screen: glam::Mat4::perspective_rh_gl(
                std::f32::consts::PI / 2.0,
                WIDTH as f32 / HEIGHT as f32,
                1.0,
                10.0,
            )
            .into(),
            time,
        }
    }
}

// SDL glue

fn main() {
    simple_logger::init().unwrap();

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Half of deferred shading", 1024, 768)
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

            match event {
                Quit { .. } => return,
                _ => {}
            }
        }

        demo.draw().unwrap();
        window.gl_swap_window();
    }
}
