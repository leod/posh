# posh

The aim of `posh` is to bring joy back to graphics programming (for myself, and
maybe others).

## Example

TODO

## Scope

In order to simplify things, we intentionally limit the scope of `posh`:

1. `posh` targets a subset of [OpenGL ES
   3.0](https://registry.khronos.org/OpenGL/specs/es/3.0/es_spec_3.0.pdf).
2. `posh` targets a subset of [GLSL ES
   3.00](https://registry.khronos.org/OpenGL/specs/es/3.0/GLSL_ES_Specification_3.00.pdf).

If the basic principle works explored by `posh` works out, we will be able to
lift these restrictions over time. A second iteration of `posh` could target
e.g. a subset of `wgpu` rather than OpenGL.

## Related Work

Check out the following awesome crates which are closely related to the aims of
`posh`.

[`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu) implements a new
compiler backend for `rustc` that generates SPIR-V, empowering users to write
shaders in normal Rust code and to reuse code between the host and shaders.
Typically, `rust-gpu` shaders are defined in separate crates. In contrast to
this, `posh` provides an EDSL that enables the definition of shaders _alongside_
host code, and its primary goal is to statically check the _interface_ between
host code and shader code.

[Shades](https://github.com/phaazon/shades), like `posh`, is an EDSL for
defining statically-typed shaders. Since there is a bit of syntactic overhead to
this, Shades provides a procedural macro with which shaders can be written.
`posh` is intentionally less powerful than Shades: it does not support mutable
variables. Due to this limitation, we hope that `posh` shaders can be succinctly
composed without a procedural macro.
