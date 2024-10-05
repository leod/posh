mod block;
mod fs_interface;
mod uniform;
mod vs_interface;

pub use block::{Block, BlockDom};
pub use fs_interface::{FragmentVisitor, FsInterface, FsInterfaceDom};
pub use uniform::{
    Uniform, UniformDom, UniformNonUnit, UniformUnion, UniformVisitor,
};
pub use vs_interface::{VertexField, VertexVisitor, VsInterface, VsInterfaceDom};
