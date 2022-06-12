use posh::{lang::Ident, posh, value::Scalar, var, vec3, GenValue, Sampler2, Value, Vec4, F32};

fn foo<'a>(x: F32<'a>, y: F32<'a>) -> F32<'a> {
    let z = var(x * y);
    let w: F32 = var(1.0 + y + x + 1.0);

    z.eq(w).and(w.eq(1.0)).ternary(3.0 * z * 2.0, 1.0)
}

#[posh]
fn bar<'a>(x: F32<'a>) -> F32<'a> {
    x.eq(5.0).ternary(x.atan2(2.0), -1.0)
}

#[posh]
fn texture_thing<'a>(sampler: Sampler2<'a>) -> Vec4<'a, f32> {
    let c = var(sampler.load(vec3(2.0, 1.0, bar(3.0))));
    sampler.load(vec3(c.z, 2.0 * c.y, foo(1.0, 2.0)).normalize() / 5.0)
}

fn main() {
    let sampler = Sampler2::from_ident(Ident::new("bla")); // hack
    let result = texture_thing(sampler);

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
