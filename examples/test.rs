use posh::{posh, Value};

#[posh]
fn foo(x: posh::F32, y: posh::F32) -> posh::F32 {
    let z = var(x * y);
    let w = var(y + x + 1.0);

    ternary(and(z.eq(w), z.eq(1.0)), z * 2.0, 1.0)
}

#[posh]
fn bar(x: posh::F32) -> posh::F32 {
    ternary(x.eq(5.0), x.atan2(2.0), -1.0)
}

#[posh]
fn baz() -> posh::Vec3<f32> {
    let dings = var(vec3(foo(1.0, 2.0), bar(42.0), -1.0));
    let thing = var(vec3(dings.z, dings.x * 3.0, dings.y));
    thing * (dings.normalize() / 5.0)
}

fn main() {
    let result = baz();
    //println!("{:#?}", result);

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
