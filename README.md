# posh

`posh` is a library for fully type-safe graphics programming.

TODO

## Example

TODO

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
