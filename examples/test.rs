use posh::{
    expose::compile::compile1, lang::defs::Defs, lang::show::show_defs, vec2, GenValue, Rep,
    ScalarType,
};

#[posh::def]
fn triplet<T: ScalarType>(t: Rep<T>) -> posh::Vec3<T> {
    posh::vec3(t, t, t)
}

#[posh::def]
fn foo(x: Rep<f32>, y: Rep<f32>) -> Rep<f32> {
    let z = posh::var(x * y);
    let w = posh::var(1.0 + y + x + 1.0);

    z.eq(w).and(w.eq(1.0)).branch(3.0 * z * 2.0, 1.0)
}

#[posh::def]
fn bar(x: Rep<f32>) -> Rep<f32> {
    let ints = triplet::<u32>(1.into());
    let floats = triplet(2.0.into());

    floats.x * ints.y.eq(2u32).branch(-1.0, foo(x, x))
}

#[posh::def]
fn texture_thing(sampler: posh::Sampler2) -> posh::Vec4<f32> {
    let c = posh::var(sampler.load(vec2(1.0, bar(3.0.into()))));
    sampler.load(vec2(2.0 * c.y, foo(1.0.into(), 2.0.into())).normalize() / 5.0)
}

fn main() {
    let func_def = compile1(texture_thing).unwrap();

    println!("{}", show_defs(&Defs::from_func_def(&func_def)));
}
