use posh::{
    sl::{self, Value},
    Numeric, Sl, Uniform, UniformDomain, VertexDomain,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

#[derive(Uniform)]
struct MyUniform2<D: UniformDomain = Sl> {
    x: D::Vec2<f32>,
}

/*/
#[derive(Uniform)]
struct MyUniform<D: UniformDomain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform2<D>,
}
*/

struct MyVertex<D: VertexDomain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

fn main() {}
