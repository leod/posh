mod utils;

use instant::Instant;

use posh::{gl, sl, Block, BlockDom, FsDom, FsInterface, Gl, Sl, Uniform, UniformDom};

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

#[derive(Clone, Copy, Uniform)]
pub struct SceneSamplers<D: UniformDom> {
    albedo: D::ColorSampler2d<sl::Vec3>,
    world_normal: D::ColorSampler2d<sl::Vec3>,
    world_pos: D::ColorSampler2d<sl::Vec3>,
}

#[derive(Clone, Copy, FsInterface)]
pub struct SceneAttachments<D: FsDom> {
    albedo: D::ColorAttachment<sl::Vec3>,
    world_normal: D::ColorAttachment<sl::Vec3>,
    world_pos: D::ColorAttachment<sl::Vec3>,
}

impl SceneAttachments<Gl> {
    fn as_scene_samplers(&self) -> SceneSamplers<Gl> {
        let params = gl::Sampler2dParams::linear();

        SceneSamplers {
            albedo: self.albedo.as_color_sampler(params),
            world_normal: self.world_normal.as_color_sampler(params),
            world_pos: self.world_pos.as_color_sampler(params),
        }
    }
}

// Shaders

mod scene_pass {
    use posh::{sl, Sl, ToSl};

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

    pub fn vertex_shader(
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
            clip_pos: screen_pos,
            interp: SceneAttachments {
                albedo: sl::vec3(1.0, 0.0, 0.0),
                world_pos,
                world_normal,
            },
        }
    }

    pub fn fragment_shader(_: (), interp: SceneAttachments<Sl>) -> SceneAttachments<Sl> {
        interp
    }
}

mod present_pass {
    use posh::{sl, Sl, ToSl};

    use crate::SceneSamplers;

    pub const SQUARE_POSITIONS: [glam::Vec2; 6] = [
        glam::vec2(1., 1.),
        glam::vec2(1., -1.),
        glam::vec2(-1., 1.),
        glam::vec2(-1., 1.),
        glam::vec2(-1., -1.),
        glam::vec2(1., -1.),
    ];

    pub fn vertex_shader(input: sl::VsInput<()>) -> sl::VsOutput<sl::Vec2> {
        let position = SQUARE_POSITIONS.to_sl().get(input.vertex_id);

        sl::VsOutput {
            clip_pos: position.extend(0.0).extend(1.0),
            interp: (position + 1.0) / 2.0,
        }
    }

    pub fn fragment_shader(samplers: SceneSamplers<Sl>, uv: sl::Vec2) -> sl::Vec4 {
        let albedo = samplers.albedo.sample(sl::vec2(uv.x * 3.0, uv.y));
        let world_normal = samplers
            .world_normal
            .sample(sl::vec2((uv.x - 2.0) * 3.0, uv.y));
        let world_pos = samplers
            .world_pos
            .sample(sl::vec2((uv.x - 1.0) * 3.0, uv.y));

        uv.x.lt(1.0 / 3.0)
            .then(albedo)
            .else_then(uv.x.lt(2.0 / 3.0), world_normal)
            .otherwise(world_pos)
            .extend(1.0)
    }
}

// Host code

struct Demo {
    gl: gl::Context,
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
            gl: gl.clone(),
            scene_program: gl
                .create_program(scene_pass::vertex_shader, scene_pass::fragment_shader)?,
            present_program: gl
                .create_program(present_pass::vertex_shader, present_pass::fragment_shader)?,
            globals: gl.create_uniform_buffer(Globals::new(0.0), StreamDraw)?,
            scene_attachments,
            depth_texture: gl.create_depth_texture_2d(gl::DepthImage::f32_zero([WIDTH, HEIGHT]))?,
            start_time: Instant::now(),
        })
    }

    pub fn draw(&mut self) -> Result<(), gl::DrawError> {
        let time = Instant::now().duration_since(self.start_time).as_secs_f32();
        self.globals.set(Globals::new(time));

        self.gl.clear(
            gl::Framebuffer::default(),
            gl::ClearParams {
                color: Some([0.0; 4]),
                depth: Some(1.0),
                ..Default::default()
            },
        )?;

        self.scene_program
            .with_uniforms(self.globals.as_binding())
            .with_framebuffer(
                self.depth_texture
                    .as_depth_attachment()
                    .with_color(self.scene_attachments.clone()),
            )
            .with_params(gl::DrawParams::new().with_depth_test(gl::Comparison::Less))
            .draw(gl::PrimitiveMode::Triangles.as_vertex_spec_with_range(0..36))?;

        self.present_program
            .with_uniforms(self.scene_attachments.as_scene_samplers())
            .draw(gl::PrimitiveMode::Triangles.as_vertex_spec_with_range(0..6))?;

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

// Platform glue

fn main() {
    utils::run_demo("Rendering into multiple textures", Demo::new, Demo::draw);
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub async fn run() {
    utils::init_wasm().await;

    #[allow(clippy::main_recursion)]
    main();
}
