use fsl::{fsl, Value};

#[fsl]
fn foo(x: fsl::F32, y: fsl::F32) -> fsl::F32 {
    let z = var(x * y);
    let w = var(y + x + 1.0);

    ternary(and(z.eq(w), z.eq(1.0)), z * 2.0, 1.0)
}

#[fsl]
fn bar(x: fsl::F32) -> fsl::F32 {
    ternary(x.eq(5.0), 1.0, -1.0)
}

#[fsl]
fn baz() -> fsl::Vec3<f32> {
    let dings = var(vec3(foo(1.0, 2.0), bar(42.0), -1.0));
    let thing = var(vec3(dings.z, dings.x * 3.0, dings.y));
    thing * (dings.normalize() / 5.0)
}

fn main() {
    let result = baz();
    //println!("{:#?}", result);

    if let fsl::lang::Expr::Call(expr) = result.expr() {
        if let fsl::lang::Func::UserDefined(func) = expr.func {
            println!("{}", fsl::lang::show::show_user_defined_funcs(&func));
        }
    }
}
