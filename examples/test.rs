use posh::{lang::Ident, vec2, GenValue, MapToExpr, Rep};

#[posh::def]
fn foo(x: Rep<f32>, y: Rep<f32>) -> Rep<f32> {
    let z = posh::var(x * y);
    let w = posh::var(1.0 + y + x + 1.0);

    z.eq(w).and(w.eq(1.0)).ternary(3.0 * z * 2.0, 1.0)
}

#[posh::def]
fn bar(x: Rep<f32>) -> Rep<f32> {
    x.eq(5.0).ternary(x.atan2(2.0), -1.0)
}

#[posh::def]
fn texture_thing(sampler: posh::Sampler2) -> posh::Vec4<f32> {
    let c = posh::var(sampler.load(vec2(1.0, bar(3.0.into()))));
    sampler.load(vec2(2.0 * c.y, foo(1.0.into(), 2.0.into())).normalize() / 5.0)
}

fn main() {
    let sampler = posh::Sampler2::from_ident(Ident::new("bla")); // hack
    let result = texture_thing(sampler);

    if let posh::lang::Expr::Call(expr) = result.expr() {
        if let posh::lang::Func::UserDefined(func) = expr.func {
            println!("{}", posh::lang::show::show_user_defined_funcs(&func));
        }
    }
}
