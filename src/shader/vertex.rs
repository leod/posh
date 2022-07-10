use crate::Value;

pub trait Vertex: Value {}

pub trait VInputs: Value {}

impl<V: Vertex> VInputs for V {}

impl<V1: Vertex, V2: Vertex> VInputs for (V1, V2) {}

pub trait VOutputs: Value {}

pub trait FOutputs: Value {}
