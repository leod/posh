use expect_test::{expect, Expect};

use posh::{
    sl::{self, program_def::ProgramDef, transpile::transpile_to_program_def},
    Block, BlockDom, FsDom, FsInterface, Sl, Uniform, UniformDom, VsDom, VsInterface,
};

fn check_program_def(actual: ProgramDef, expect_vs: Expect, expect_fs: Expect) {
    expect_vs.assert_eq(&actual.vertex_shader_source);
    expect_fs.assert_eq(&actual.fragment_shader_source);
}

#[derive(Copy, Clone, Block)]
#[repr(C)]
pub struct MyBlock<D: BlockDom> {
    scale: D::F32,
}

#[test]
fn test_simple() {
    fn vertex_shader(block: MyBlock<Sl>, pos: sl::Vec4) -> sl::Vec4 {
        block.scale * pos
    }

    fn fragment_shader((): ()) -> sl::Vec4 {
        sl::Vec4::ZERO
    }

    let actual =
        transpile_to_program_def::<MyBlock<Sl>, _, _, _, _>(vertex_shader, fragment_shader);

    check_program_def(
        actual,
        expect![[r#"
            #version 300 es

            precision highp float;
            precision highp int;
            precision highp sampler2DShadow;
            precision highp sampler2D;

            struct MyBlock_Posh0 {
                float scale;
            };

            layout(std140) uniform uniforms_posh_block {
                MyBlock_Posh0 uniforms;
            };

            in vec4 vertex_input;

            void main() {
                gl_Position = (uniforms.scale * vertex_input);
            }
        "#]],
        expect![[r#"
            #version 300 es

            precision highp float;
            precision highp int;
            precision highp sampler2DShadow;
            precision highp sampler2D;

            struct MyBlock_Posh0 {
                float scale;
            };

            layout(std140) uniform uniforms_posh_block {
                MyBlock_Posh0 uniforms;
            };

            layout(location = 0) out vec4 fragment_output;

            void main() {
                fragment_output = vec4(0.0, 0.0, 0.0, 0.0);
            }
        "#]],
    );
}

#[derive(Copy, Clone, sl::Value, sl::Interpolant)]
pub struct MyInterpolant {
    uv: sl::Vec2,
    scale: sl::F32,
    instance_id: sl::U32,
}

#[derive(Copy, Clone, Block)]
#[repr(C)]
pub struct MyGlobals<D: BlockDom> {
    ambient: D::Vec4,
    world_to_clip: D::Mat4,
    block: MyBlock<D>,
}

#[derive(Copy, Clone, Uniform)]
pub struct MyUniform<D: UniformDom> {
    sampler: D::ColorSampler2d<sl::Vec3>,
    globals: D::Block<MyGlobals<Sl>>,
}

#[derive(Copy, Clone, VsInterface)]
pub struct MyVsInterface<D: VsDom> {
    world_pos: D::Block<sl::Vec2>,
    color: D::Block<sl::Vec4>,
    block: D::Block<MyBlock<Sl>>,
}

#[derive(Copy, Clone, FsInterface)]
pub struct MyFsInterface<D: FsDom> {
    albedo: D::ColorAttachment<sl::Vec4>,
    normal: D::ColorAttachment<sl::Vec3>,
    glow: D::ColorAttachment<sl::F32>,
}

#[test]
fn test_more_types() {
    // This is a nonsensical shader. The purpose of it is to test some features
    // in combination, such as nested types and shader inputs/outputs.

    fn vertex_shader(
        uniform: MyUniform<Sl>,
        sl::VsInput {
            vertex,
            vertex_id,
            instance_id,
            ..
        }: sl::VsInput<MyVsInterface<Sl>>,
    ) -> sl::VsOutput<(sl::Vec4, MyInterpolant)> {
        let z = vertex_id.eq(instance_id).then(1.0).otherwise(0.5);
        let clip_pos = uniform.globals.world_to_clip * vertex.world_pos.extend(z).extend(1.0);

        sl::VsOutput {
            clip_pos,
            interp: (
                vertex.color,
                MyInterpolant {
                    uv: vertex.world_pos * 0.01,
                    scale: vertex.block.scale,
                    instance_id,
                },
            ),
        }
    }

    fn fragment_shader(
        uniform: MyUniform<Sl>,
        input @ sl::FsInput {
            interp: (color, interp),
            fragment_coord,
            front_facing,
            point_coord,
            ..
        }: sl::FsInput<(sl::Vec4, MyInterpolant)>,
    ) -> sl::FullFsOutput<MyFsInterface<Sl>> {
        let sample = uniform.sampler.sample(interp.uv);
        let albedo = sample
            .z
            .eq(0.0)
            .then_discard(input)
            .otherwise(color.xyz() * sample);

        let fragment = MyFsInterface {
            albedo: albedo.extend(color.w) + fragment_coord,
            normal: front_facing.then(sl::Vec3::Y).otherwise(sl::Vec3::Z),
            glow: interp.scale + point_coord.x + point_coord.y,
        };

        sl::FullFsOutput {
            fragment,
            fragment_depth: Some(interp.scale),
        }
    }

    let actual =
        transpile_to_program_def::<MyUniform<Sl>, _, _, _, _>(vertex_shader, fragment_shader);

    // FIXME: Why is `common_field_base` not collapsing the vector expressions
    // below?
    // ```
    // var_2 = (vec3(vertex_output_T6.x, vertex_output_T6.y, vertex_output_T6.z) * vec3(var_0.x, var_0.y, var_0.z));
    // ```
    check_program_def(
        actual,
        expect![[r#"
            #version 300 es

            precision highp float;
            precision highp int;
            precision highp sampler2DShadow;
            precision highp sampler2D;

            struct MyBlock_Posh0 {
                float scale;
            };
            struct MyGlobals_Posh1 {
                vec4 ambient;
                mat4 world_to_clip;
                MyBlock_Posh0 block;
            };

            uniform sampler2D uniforms_sampler;
            layout(std140) uniform uniforms_globals_posh_block {
                MyGlobals_Posh1 uniforms_globals;
            };

            in vec2 vertex_input_world_pos;
            in vec4 vertex_input_color;
            in float vertex_input_block_scale;
            smooth out vec4 vertex_output_T6;
            smooth out vec2 vertex_output_T7_uv;
            smooth out float vertex_output_T7_scale;
            flat out uint vertex_output_T7_instance_id;

            void main() {
                float var_0 = ((uint(gl_VertexID) == uint(gl_InstanceID))) ? (1.0) : (0.5);
                gl_Position = (uniforms_globals.world_to_clip * vec4(vertex_input_world_pos.x, vertex_input_world_pos.y, var_0, 1.0));
                vertex_output_T6 = vertex_input_color;
                vertex_output_T7_uv = (vertex_input_world_pos * 0.01);
                vertex_output_T7_scale = vertex_input_block_scale;
                vertex_output_T7_instance_id = uint(gl_InstanceID);
            }
        "#]],
        expect![[r#"
            #version 300 es

            precision highp float;
            precision highp int;
            precision highp sampler2DShadow;
            precision highp sampler2D;

            struct MyBlock_Posh0 {
                float scale;
            };
            struct MyGlobals_Posh1 {
                vec4 ambient;
                mat4 world_to_clip;
                MyBlock_Posh0 block;
            };

            uniform sampler2D uniforms_sampler;
            layout(std140) uniform uniforms_globals_posh_block {
                MyGlobals_Posh1 uniforms_globals;
            };

            smooth in vec4 vertex_output_T6;
            smooth in vec2 vertex_output_T7_uv;
            smooth in float vertex_output_T7_scale;
            flat in uint vertex_output_T7_instance_id;
            layout(location = 0) out vec4 fragment_output_albedo;
            layout(location = 1) out vec3 fragment_output_normal;
            layout(location = 2) out float fragment_output_glow;

            void main() {
                vec4 var_0 = texture(uniforms_sampler, vertex_output_T7_uv);
                vec3 var_2;
                if ((var_0.z == 0.0)) {
                    discard;
                } else {
                    var_2 = (vec3(vertex_output_T6.x, vertex_output_T6.y, vertex_output_T6.z) * vec3(var_0.x, var_0.y, var_0.z));
                }
                vec3 var_3 = (gl_FrontFacing) ? (vec3(0.0, 1.0, 0.0)) : (vec3(0.0, 0.0, 1.0));
                fragment_output_albedo = (vec4(var_2.x, var_2.y, var_2.z, vertex_output_T6.w) + gl_FragCoord);
                fragment_output_normal = var_3;
                fragment_output_glow = ((vertex_output_T7_scale + gl_PointCoord.x) + gl_PointCoord.y);
                gl_FragDepth = vertex_output_T7_scale;
            }
        "#]],
    );
}
