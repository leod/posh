use posh::{
    gl::{
        DefaultFramebuffer, DrawParams, GeometryType, Program, UniformBufferBinding, VertexStream,
    },
    sl::{self, Object, ToValue, Value},
    Domain, FragmentInterface, Gl, Numeric, Primitive, ResourceDomain, ResourceInterface, Sl,
    Uniform, Vertex, VertexDomain, VertexInputRate, VertexInterface,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

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
    vertex: D::Vertex<MyVertex>,
    instance: D::Vertex<MyNestedVertex>,
}

#[derive(ResourceInterface)]
struct MyResourceIface<D: ResourceDomain = Sl> {
    uniform: D::Uniform<MyUniform1>,
}

#[derive(ResourceInterface)]
struct MyResourceIface2<D: ResourceDomain = Sl> {
    uniformxy: D::Uniform<MyUniform1>,
    bla: MyResourceIface<D>,
    zzz: D::Uniform<MyUniform1>,
}

#[derive(ResourceInterface)]
struct GenericResourceIface<R, D: ResourceDomain = Sl>
where
    R: ResourceInterface<Sl>,
{
    uniformxy: D::Uniform<MyUniform1>,
    x: D::Compose<R>,
}

fn draw<R: ResourceInterface<Sl>>(
    program: &Program<GenericResourceIface<R>, sl::Vec2<f32>, sl::Vec4<f32>>,
    xy: UniformBufferBinding<MyUniform1>,
    x: R::InGl,
) {
    let resources = GenericResourceIface { uniformxy: xy, x };
    let vertices: VertexStream<sl::Vec2<f32>, ()> = todo!();

    program.draw(
        resources,
        &vertices,
        GeometryType::Triangles,
        &DefaultFramebuffer,
        &DrawParams {},
    );
}

struct MyVisitor;

impl posh::internal::VertexInterfaceVisitor<Sl> for MyVisitor {
    fn accept<V: Vertex<Sl>>(&mut self, path: &str, input_rate: VertexInputRate, vertex: &V) {
        println!("vertex iface path={path}: {:?}", V::attributes(path));
    }
}

impl posh::internal::ResourceInterfaceVisitor<Sl> for MyVisitor {
    fn accept_sampler2d<T: Numeric>(
        &mut self,
        path: &str,
        vertex: &<Sl as ResourceDomain>::Sampler2d<T>,
    ) {
    }

    fn accept_uniform<U: Uniform<Sl, InSl = U>>(
        &mut self,
        path: &str,
        vertex: &<Sl as ResourceDomain>::Uniform<U>,
    ) {
        println!("resource uniform path={path}");
    }
}

fn create_program<R, V, F>() -> Program<R, V, F>
where
    R: ResourceInterface<Sl>,
    V: VertexInterface<Sl>,
    F: FragmentInterface<Sl>,
{
    Program::new::<sl::Vec4<f32>>(todo!(), todo!())
}

fn main() {
    println!("{:#?}", <MyVertex::<Sl> as Vertex<Sl>>::attributes("foo"));
    println!(
        "{:#?}",
        <MyNestedVertex::<Gl> as Vertex<Gl>>::attributes("bar")
    );

    let vertex = <MyNestedVertex<Sl> as Vertex<Sl>>::shader_input("bar");

    println!("{:#?}", vertex.y.x.expr());

    //let vertex = <MyNestedVertex<Gl> as Vertex<Gl>>::shader_input("bar");

    MyVertexIface::shader_input("blub").visit(&mut MyVisitor);
    MyResourceIface2::shader_input("blab").visit("blee", &mut MyVisitor);

    let program: Program<MyResourceIface2, MyVertexIface, sl::Vec4<f32>> =
        Program::new::<sl::Vec4<f32>>(todo!(), todo!());

    let program: Program<MyResourceIface2, MyVertexIface, sl::Vec4<f32>> = create_program();

    let resources: MyResourceIface2<Gl> = todo!();
    let vertices: VertexStream<MyVertexIface, ()> = todo!();

    program.draw(
        resources,
        &vertices,
        GeometryType::Lines,
        &DefaultFramebuffer,
        &DrawParams {},
    );
}
