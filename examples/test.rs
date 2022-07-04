use posh::{lang::Ident, var, vec3, GenVal, Sampler2, Val, Value, Vec4};

#[posh::def]
fn foo(x: Val<f32>, y: Val<f32>) -> Val<f32> {
    let z = var(x * y);
    let w = var(1.0 + y + x + 1.0);

    z.eq(w).and(w.eq(1.0)).ternary(3.0 * z * 2.0, 1.0)
}

#[posh::def]
fn bar(x: Val<f32>) -> Val<f32> {
    x.eq(5.0).ternary(x.atan2(2.0), -1.0)
}

#[posh::def]
fn texture_thing(sampler: Sampler2) -> Vec4<f32> {
    let c = var(sampler.load(vec3(1.0, 1.0, bar(3.0))));
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
