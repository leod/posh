use std::f32::consts::PI;

use expect_test::{expect, Expect};

use crate::{
    sl::{self, primitives::value_arg, Derivatives, FsInput, Value},
    Sl, ToSl, VsInterface,
};

use super::{
    scope_form::ScopeForm, struct_registry::StructRegistry, var_form::VarForm, write_scope,
    WriteFuncContext,
};

fn transpile_expr<V: Value>(value: V) -> String {
    let struct_registry = StructRegistry::new(&[], None.into_iter());
    let var_form = VarForm::new(&struct_registry, &[value.expr()]);
    let scope_form = ScopeForm::new(&var_form);
    let write_context = WriteFuncContext {
        struct_registry: &struct_registry,
        scope_form: &scope_form,
        depth: 0,
    };

    let mut str = String::new();
    write_scope(&mut str, write_context, scope_form.root_scope()).unwrap();
    str += &format!("{}", var_form.simplified_roots()[0]);

    str
}

fn check_expr<V: Value>(value: impl ToSl<Output = V>, expect: Expect) {
    let actual = transpile_expr(value.to_sl());

    expect.assert_eq(&actual);
}

#[test]
fn test_literal_f32() {
    check_expr(4.0, expect![[r#"4.0"#]]);
    check_expr(-7000.2, expect![[r#"-7000.2"#]]);
    check_expr(PI, expect![[r#"3.1415927"#]]);
    check_expr(f32::MAX, expect!["3.4028235e38"]);
    check_expr(f32::MIN, expect!["-3.4028235e38"]);
    check_expr(f32::MIN_POSITIVE, expect!["1.1754944e-38"]);
    check_expr(0.0, expect!["0.0"]);
    check_expr(-0.0, expect!["-0.0"]);

    // FIXME: Code generation for infinities and NaN is wrong. GLSL does not
    // have constants for these. Newer versions of GLSL seem to specify the
    // result of `1.0 / 0.0`, but we cannot rely on this yet. Should we just
    // panic in codegen when encountering infinities or NaN? Adjust the test
    // output once we have fixed this issue.
    check_expr(f32::NEG_INFINITY, expect!["-inf"]);
    check_expr(f32::INFINITY, expect!["inf"]);
    check_expr(f32::NAN, expect!["NaN"]);
}

#[test]
fn test_literal_i32() {
    check_expr(0, expect![[r#"0"#]]);
    check_expr(-0, expect![[r#"0"#]]);
    check_expr(4, expect![[r#"4"#]]);
    check_expr(-1234, expect!["-1234"]);
    check_expr(i32::MIN, expect!["-2147483648"]);
    check_expr(i32::MAX, expect!["2147483647"]);
}

#[test]
fn test_literal_u32() {
    check_expr(0u32, expect!["0u"]);
    check_expr(4u32, expect!["4u"]);
    check_expr(1234u32, expect!["1234u"]);
    check_expr(u32::MIN, expect!["0u"]);
    check_expr(u32::MAX, expect!["4294967295u"]);
}

#[test]
fn test_literal_bool() {
    check_expr(false, expect!["false"]);
    check_expr(true, expect!["true"]);
}

#[test]
fn test_array() {
    let arr_1: [f32; 1] = [100.0];
    let arr_2: [f32; 1] = [200.0];
    let arr_3: [u32; 6] = std::array::from_fn(|i| i as u32);

    let cond: sl::Bool = value_arg("cond");
    let i: sl::U32 = value_arg("i");

    check_expr(
        arr_1.to_sl().get(0u32),
        expect![[r#"
        float[1] var_0 = float[1](100.0);
        var_0[0u]"#]],
    );
    check_expr(
        arr_1.to_sl().get(i),
        expect![[r#"
            float[1] var_0 = float[1](100.0);
            var_0[i]"#]],
    );
    check_expr(
        {
            let a = arr_1.to_sl();

            a.to_sl().get(i.ge(a.len()).then(a.len() - 1).otherwise(i))
        },
        expect![[r#"
            float[1] var_0 = float[1](100.0);
            uint var_1 = ((i >= 1u)) ? ((1u - 1u)) : (i);
            var_0[var_1]"#]],
    );
    check_expr(
        {
            let result = arr_1.to_sl().get(1u32).as_u32();
            let result = arr_1.to_sl().get(result).as_u32();

            arr_3.to_sl().get(result)
        },
        expect![[r#"
            uint[6] var_0 = uint[6](0u, 1u, 2u, 3u, 4u, 5u);
            float[1] var_1 = float[1](100.0);
            float[1] var_2 = float[1](100.0);
            var_0[uint(var_1[uint(var_2[1u])])]"#]],
    );
    check_expr(
        cond.then(arr_1).otherwise(arr_2).get(i),
        expect![[r#"
        float[1] var_2;
        if (cond) {
            float[1] var_0 = float[1](100.0);
            var_2 = var_0;
        } else {
            float[1] var_1 = float[1](200.0);
            var_2 = var_1;
        }
        var_2[i]"#]],
    )
}

#[test]
fn test_simple_exprs() {
    let x: sl::F32 = value_arg("x");
    let y: sl::F32 = value_arg("y");

    check_expr(x * x, expect!["(x * x)"]);
    check_expr(
        (x * x.atan2(2.5) + y) / 4.0 * sl::vec2(x, y),
        expect!["((((x * atan(x, 2.5)) + y) / 4.0) * vec2(x, y))"],
    );
}

#[test]
fn test_branch() {
    let cond: sl::Bool = value_arg("cond");
    let x: sl::F32 = value_arg("x");
    let y: sl::F32 = value_arg("y");

    check_expr(
        cond.then(x).otherwise(y),
        expect![[r#"
            float var_0 = (cond) ? (x) : (y);
            var_0"#]],
    );
    check_expr(
        cond.then(x.gt(y).then(x).otherwise(y))
            .otherwise(x.gt(y).then(y).otherwise(x)),
        expect![[r#"
            float var_2;
            if (cond) {
                float var_0 = ((x > y)) ? (x) : (y);
                var_2 = var_0;
            } else {
                float var_1 = ((x > y)) ? (y) : (x);
                var_2 = var_1;
            }
            var_2"#]],
    )
}

#[test]
fn test_branch_chain() {
    let mut x: sl::F32 = value_arg("x");
    let mut y = x;

    for i in 0..30 {
        x = x.eq(y).branch(-x, x / i as f32);

        if i % 3 == 0 {
            y = x;
        }
    }

    check_expr(
        x,
        expect![[r#"
            float var_0 = ((x == x)) ? (- x) : ((x / 0.0));
            float var_1 = ((var_0 == var_0)) ? (- var_0) : ((var_0 / 1.0));
            float var_2 = ((var_1 == var_0)) ? (- var_1) : ((var_1 / 2.0));
            float var_3 = ((var_2 == var_0)) ? (- var_2) : ((var_2 / 3.0));
            float var_4 = ((var_3 == var_3)) ? (- var_3) : ((var_3 / 4.0));
            float var_5 = ((var_4 == var_3)) ? (- var_4) : ((var_4 / 5.0));
            float var_6 = ((var_5 == var_3)) ? (- var_5) : ((var_5 / 6.0));
            float var_7 = ((var_6 == var_6)) ? (- var_6) : ((var_6 / 7.0));
            float var_8 = ((var_7 == var_6)) ? (- var_7) : ((var_7 / 8.0));
            float var_9 = ((var_8 == var_6)) ? (- var_8) : ((var_8 / 9.0));
            float var_10 = ((var_9 == var_9)) ? (- var_9) : ((var_9 / 10.0));
            float var_11 = ((var_10 == var_9)) ? (- var_10) : ((var_10 / 11.0));
            float var_12 = ((var_11 == var_9)) ? (- var_11) : ((var_11 / 12.0));
            float var_13 = ((var_12 == var_12)) ? (- var_12) : ((var_12 / 13.0));
            float var_14 = ((var_13 == var_12)) ? (- var_13) : ((var_13 / 14.0));
            float var_15 = ((var_14 == var_12)) ? (- var_14) : ((var_14 / 15.0));
            float var_16 = ((var_15 == var_15)) ? (- var_15) : ((var_15 / 16.0));
            float var_17 = ((var_16 == var_15)) ? (- var_16) : ((var_16 / 17.0));
            float var_18 = ((var_17 == var_15)) ? (- var_17) : ((var_17 / 18.0));
            float var_19 = ((var_18 == var_18)) ? (- var_18) : ((var_18 / 19.0));
            float var_20 = ((var_19 == var_18)) ? (- var_19) : ((var_19 / 20.0));
            float var_21 = ((var_20 == var_18)) ? (- var_20) : ((var_20 / 21.0));
            float var_22 = ((var_21 == var_21)) ? (- var_21) : ((var_21 / 22.0));
            float var_23 = ((var_22 == var_21)) ? (- var_22) : ((var_22 / 23.0));
            float var_24 = ((var_23 == var_21)) ? (- var_23) : ((var_23 / 24.0));
            float var_25 = ((var_24 == var_24)) ? (- var_24) : ((var_24 / 25.0));
            float var_26 = ((var_25 == var_24)) ? (- var_25) : ((var_25 / 26.0));
            float var_27 = ((var_26 == var_24)) ? (- var_26) : ((var_26 / 27.0));
            float var_28 = ((var_27 == var_27)) ? (- var_27) : ((var_27 / 28.0));
            float var_29 = ((var_28 == var_27)) ? (- var_28) : ((var_28 / 29.0));
            var_29"#]],
    );
}

#[test]
fn test_branch_complex_nesting() {
    let cond: sl::Bool = value_arg("cond");
    let x: sl::F32 = value_arg("x");
    let y: sl::F32 = value_arg("y");

    let w = x.sqrt();

    let result = x
        .lt(y)
        .or(cond)
        .then({
            let temp = (w + x).as_i32();

            cond.then(temp + 1).otherwise({
                let blub = temp * 2;

                blub + 1000 * blub
            })
        })
        .else_then(!cond, 0)
        .else_then(true, 1)
        .else_then(true, {
            let blub = (w * w).as_i32();

            (blub & 100) - (blub | 100)
        })
        .otherwise(3)
        / 2;

    // TODO: We could simplify the generated code by generating `else if`
    // instead of nesting.
    check_expr(
        result,
        expect![[r#"
            float var_0 = sqrt(x);
            int var_8;
            if (((x < y) || cond)) {
                int var_1 = int((var_0 + x));
                int var_3;
                if (cond) {
                    var_3 = (var_1 + 1);
                } else {
                    int var_2 = (var_1 * 2);
                    var_3 = (var_2 + (1000 * var_2));
                }
                var_8 = var_3;
            } else {
                int var_7;
                if (! cond) {
                    var_7 = 0;
                } else {
                    int var_6;
                    if (true) {
                        var_6 = 1;
                    } else {
                        int var_5;
                        if (true) {
                            int var_4 = int((var_0 * var_0));
                            var_5 = ((var_4 & 100) - (var_4 | 100));
                        } else {
                            var_5 = 3;
                        }
                        var_6 = var_5;
                    }
                    var_7 = var_6;
                }
                var_8 = var_7;
            }
            (var_8 / 2)"#]],
    );
}

#[test]
fn test_branch_nesting() {
    let result = (0..5)
        .scan(value_arg("x"), |x, i| {
            let r: sl::F32 = *x;

            *x = r * r + i as f32;

            Some(r)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .fold(123.0.to_sl(), |result, x| {
            x.gt(0.0).then(x).otherwise(result)
        });

    check_expr(
        result,
        expect![[r#"
            float var_8;
            if ((x > 0.0)) {
                var_8 = x;
            } else {
                float var_0 = ((x * x) + 0.0);
                float var_7;
                if ((var_0 > 0.0)) {
                    var_7 = var_0;
                } else {
                    float var_1 = ((var_0 * var_0) + 1.0);
                    float var_6;
                    if ((var_1 > 0.0)) {
                        var_6 = var_1;
                    } else {
                        float var_2 = ((var_1 * var_1) + 2.0);
                        float var_5;
                        if ((var_2 > 0.0)) {
                            var_5 = var_2;
                        } else {
                            float var_3 = ((var_2 * var_2) + 3.0);
                            float var_4 = ((var_3 > 0.0)) ? (var_3) : (123.0);
                            var_5 = var_4;
                        }
                        var_6 = var_5;
                    }
                    var_7 = var_6;
                }
                var_8 = var_7;
            }
            var_8"#]],
    );
}

#[test]
fn test_discard() {
    let input = fs_input::<()>();
    let cond: sl::Bool = value_arg("cond");

    check_expr(
        cond.then(sl::Vec2::X).otherwise_discard(input),
        expect![[r#"
            vec2 var_1;
            if (cond) {
                var_1 = vec2(1.0, 0.0);
            } else {
                discard;
            }
            var_1"#]],
    );
    check_expr(
        cond.then_discard(input).otherwise(sl::Vec3::X),
        expect![[r#"
            vec3 var_1;
            if (cond) {
                discard;
            } else {
                var_1 = vec3(1.0, 0.0, 0.0);
            }
            var_1"#]],
    );
    check_expr(
        cond.then(2).else_then_discard(!cond, input).otherwise(3),
        expect![[r#"
            int var_2;
            if (cond) {
                var_2 = 2;
            } else {
                int var_1;
                if (! cond) {
                    discard;
                } else {
                    var_1 = 3;
                }
                var_2 = var_1;
            }
            var_2"#]],
    );
    check_expr(
        cond.then(sl::Vec4::X)
            .otherwise((!cond).then_discard(input).otherwise(sl::Vec4::Y)),
        expect![[r#"
            vec4 var_2;
            if (cond) {
                var_2 = vec4(1.0, 0.0, 0.0, 0.0);
            } else {
                vec4 var_1;
                if (! cond) {
                    discard;
                } else {
                    var_1 = vec4(0.0, 1.0, 0.0, 0.0);
                }
                var_2 = var_1;
            }
            var_2"#]],
    );
    check_expr(
        cond.then_discard::<sl::I32, _>(input)
            .else_then_discard(false, input)
            .otherwise_discard(input),
        expect![[r#"
            int var_4;
            if (cond) {
                discard;
            } else {
                int var_3;
                if (false) {
                    discard;
                } else {
                    discard;
                }
                var_4 = var_3;
            }
            var_4"#]],
    );
}

fn fs_input<W: VsInterface<Sl>>() -> FsInput<W> {
    FsInput {
        interp: W::shader_input("vertex_output"),
        fragment_coord: value_arg("gl_FragCoord"),
        front_facing: value_arg("gl_FrontFacing"),
        point_coord: value_arg("gl_PointCoord"),
        derivatives: Derivatives(()),
    }
}
