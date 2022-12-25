# posh

`posh` is an experiment: can we use Rust to statically check the type safety of
draw calls and their interaction with shader code? Can we do so without losing
much on ergonomics? `posh` approaches this problem by requiring shaders to be
defined in a functional EDSL for shaders alongside the host code that calls
them.

One of Rust's advantages is its ability to catch certain types of errors at
compile-time through static typing. In graphics programming, a common source of
error is a mismatch in the interface between host code and shader code. In fact,
draw calls can be thought of as a foreign function interface between CPU and
GPU. Thus, we believe that there is some potential in applying static type
checking to draw calls.

We aim to validate draw calls by defining shader code alongside host code and by
explicitly declaring shader interfaces in Rust. As a secondary effect, we hope
that shaders written in `posh` will become more reusable and composable than
usual shader code.

## Example

First, we need to define the types that will appear in our shader's signature.
Here, we'll define a `Uniform` and a `Vertex`. In `posh`, such types need to
have _dual_ representations in the graphics library domain `Gl` and the shader
language domain `Sl`. In this example, the host will provide vertex data as
`MyVertex<Gl>`, while the shader will access it through the dual type
`MyVertex<Sl>`.

```rust
use posh::{Domain, Sl, Uniform, Vertex, sl::ToValue};

#[derive(Clone, Copy, ToValue, Uniform)]
struct Camera<D: Domain = Sl> {
   view: D::Mat4<f32>,
   projection: D::Mat4<f32>,
}

#[derive(Clone, Copy, ToValue, Vertex)]
struct MyVertex<D: Domain = Sl> {
   pos: D::Vec3<f32>,
   color: D::Vec3<f32>,
}

```

Next, we can use these types to define a simple shader. Notice that the shader
stages are defined as simple Rust code!
```rust
use posh::sl::{FragmentOutput, Vec3, Vec4, VertexOutput};

fn vertex_stage(camera: Camera, vertex: MyVertex) -> VertexOutput<Vec3<f32>> {
   let position = camera.projection * camera.view * vertex.pos.to_vec4();

   VertexOutput {
      position,
      varying: vertex.color,
      ..Default::default()
   }
}

fn fragment_stage(varying: Vec3<f32>) -> FragmentOutput<Vec4<f32>> {
   FragmentOutput {
      fragment: varying,
      ..Default::default()
   }
}
```

Finally, we can compile our shader into a `Program<R, A, F>`. The three type
parameters signify the interface of our shader: `R` are input resources such as
uniforms and textures, `A` are input attributes, and `F` is the output fragment
type. This makes it possible to check at compile-time that the buffers passed to
draw calls match the signature of the shader.
```rust
use posh::gl::{
    Context, DefaultSurface, DrawParams, PrimitiveType, Program, UniformBuffer,
    VertexArray,
};

fn main() {
    // Create a context & fill some buffers.
    let ctx: Context = todo!();
    let camera: UniformBuffer<Camera> = todo!();
    let vertices: VertexArray<MyVertex> = todo!();

    // Compile the shader into a program:
    let program: Program<Camera, MyVertex, Vec4<f32>> =
        Program::new(&ctx, vertex_stage, fragment_stage).unwrap();

    // Finally, a draw call!
    program.draw(
        &camera,
        vertices.vertex_stream_without_elements(PrimitiveType::Triangles),
        &DefaultSurface,
        &DrawParams::default(),
    );
}
```

## Scope

In order to simplify things, we intentionally limit the scope of `posh`:

1. `posh` targets OpenGL rather than a more modern API like WebGPU.
2. `posh` targets
   [GLSL 3.30](https://registry.khronos.org/OpenGL/specs/gl/GLSLangSpec.3.30.pdf).
3. `posh` supports only a subset of OpenGL and GLSL features.

If the fundamental principle works out, we should be able to lift these
restrictions over time.

## Related Work

Check out the following awesome (and much more mature) crates which are closely
related to the aims of `posh`.

[`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu) implements a new
compiler backend for `rustc` that generates SPIR-V, empowering users to write
shaders in normal Rust code and to reuse code between the host and shaders.
Typically, `rust-gpu` shaders are defined in separate crates. In contrast to
this, `posh` is an EDSL that enables you to define shaders _alongside_ host
code, and its primary goal is to statically check the _interface_ between host
code and shader code.

[Shades](https://github.com/phaazon/shades), like `posh`, is an EDSL for
defining statically-typed shaders. Since there is a bit of syntactic overhead to
this, Shades provides a procedural macro with which shaders can be written.
`posh` is intentionally less powerful than Shades: it does not support mutable
variables. Due to this limitation, we hope that `posh` shaders can be succinctly
composed without a procedural macro.