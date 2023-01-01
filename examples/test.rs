use posh::{
    sl::{self, Object, ToValue, Value},
    Domain, Gl, Numeric, Primitive, Sl, Uniform, Vertex, VertexDomain, VertexInterface,
};

/*
#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}
*/

#[derive(Clone, Copy, ToValue)]
struct MyThang<T: Primitive, D: Domain = Sl> {
    x: sl::F32,
    y: D::Scalar<T>,
}

#[derive(Clone, Copy, ToValue)]
struct MyThunk<T: Primitive, D: Domain = Sl> {
    x: MyThang<T, D>,
}

#[derive(Clone, Copy, ToValue, Uniform, Vertex)]
struct MyUniform1<D: Domain = Sl> {
    x: D::Vec2<f32>,
    y: D::Bool,
}

#[derive(Clone, Copy, ToValue, Uniform)]
struct MyUniform2<D: Domain = Sl> {
    x: D::Vec2<f32>,
    y: MyUniform1<D>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyVertex<D: Domain = Sl> {
    x: D::F32,
    y: D::Vec2<f32>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyNestedVertex<D: Domain = Sl> {
    x: D::Scalar<f32>,
    zzz: MyUniform1<D>,
    y: D::Vec2<f32>,
}

#[derive(VertexInterface)]
struct MyVertexIface<D: VertexDomain = Sl> {
    vertex: D::Vertex<MyVertex<D>>,
    instance: D::Vertex<MyNestedVertex<D>>,
}

struct MyVisitor;

impl posh::derive_internal::VertexInterfaceVisitor<Sl> for MyVisitor {
    fn accept<V: Vertex<Sl>>(&mut self, path: &str, vertex: &V) {
        println!("vertex iface path={path}: {:?}", V::attributes(path));
    }
}

fn main() {
    /*
    println!("{:#?}", <MyVertex::<Sl> as Vertex<Sl>>::attributes("foo"));
    println!(
        "{:#?}",
        <MyNestedVertex::<Gl> as Vertex<Gl>>::attributes("bar")
    );

    let vertex = <MyNestedVertex<Sl> as Vertex<Sl>>::shader_input("bar");

    println!("{:#?}", vertex.y.x.expr());

    //let vertex = <MyNestedVertex<Gl> as Vertex<Gl>>::shader_input("bar");

    MyVertexIface::shader_input("blub").visit(&mut MyVisitor);
    */
}
