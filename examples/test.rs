use posh::{
    expose::compile::compile1,
    var_form::{
        show::{show_func_defs, show_struct_defs},
        StructDefs, VarFormFuncDefs,
    },
    vec2, GenValue, Posh, ScalarType,
};

#[posh::def]
fn triplet<T: ScalarType>(t: Posh<T>) -> (posh::Vec3<T>, Posh<u32>) {
    (posh::vec3(t, t, t), 32.into())
}

#[posh::def]
fn foo(x: Posh<f32>, y: Posh<f32>, z: (Posh<f32>, Posh<f32>)) -> Posh<f32> {
    let z = x * y - z.1;
    let w = 1.0 + y + x + 1.0;

    z.eq(w).and(w.eq(1.0)).branch(3.0 * z * 2.0, 1.0)
}

#[posh::def]
fn bar(x: Posh<f32>) -> Posh<f32> {
    let ints = triplet::<u32>(1.into()).0;
    let floats = triplet(2.0.into()).0;

    floats.x * ints.y.eq(2u32).branch(-1.0, foo(x, x, (x, x)))
}

#[posh::def]
fn texture_thing(sampler: posh::Sampler2) -> posh::Vec4<f32> {
    let c = sampler.load(vec2(1.0, bar(3.0.into())));

    let dum = foo(1.0.into(), 2.0.into(), (c.x, c.y));
    let tex_coord = vec2(2.0 * c.y, dum).normalize() / 5.0;

    sampler.load(tex_coord)
}

fn main() {
    let func_def = compile1(texture_thing).unwrap();

    let mut structs = StructDefs::new();
    let funcs = VarFormFuncDefs::from_func_def(&func_def, &mut structs);
    println!("{}", show_struct_defs(&structs));
    println!("{}", show_func_defs(&funcs));
}
