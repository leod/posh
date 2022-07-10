use crate::Value;

/// A representative of a vertex.
pub trait Vertex: Value {}

/// A representative of vertex stage input.
pub trait VInputs: Value {}

impl<V: Vertex> VInputs for V {}

impl<V1: Vertex, V2: Vertex> VInputs for (V1, V2) {}

/// A representative of vertex stage output.
pub trait VOutputs: Value {}

/// A representative of fragment stage output.
pub trait FOutputs: Value {}
