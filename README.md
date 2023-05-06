# posh

`posh` is a type-safe OpenGL ES 3.0 wrapper library with the aim of making graphics programming enjoyable.
`posh` consists of a graphics library (`posh::gl`) and a shading language (`posh::sl`), which are tightly integrated in a single crate.

## Status

`posh` is in an early alpha stage.
Use with caution.
Contributions are welcome!

## Scope

The initial scope of `posh` is limited intentionally.
It targets a subset of [OpenGL ES 3.0](https://registry.khronos.org/OpenGL/specs/es/3.0/es_spec_3.0.pdf) and [GLSL ES 3.00](https://registry.khronos.org/OpenGL/specs/es/3.0/GLSL_ES_Specification_3.00.pdf).

These restrictions can be lifted over time. A second iteration of `posh` could e.g. target a subset of `wgpu` rather than OpenGL.

## Related Work

The following awesome crates are closely related to the aims of `posh`:

- [Shades](https://github.com/phaazon/shades) is an EDSL for statically-typed shaders.

- [`rust-gpu`](https://github.com/EmbarkStudios/rust-gpu) is a compiler backend for `rustc` that generates SPIR-V.

The main difference to these is that `posh` tightly integrates its shading language (an embedded domain-specific language) with its graphics library in a single crate.