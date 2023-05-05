mod block;
mod fs_interface;
mod uniform_interface;
mod vs_interface;

pub use block::{Block, BlockDom};
pub use fs_interface::{FragmentVisitor, FsInterface, FsInterfaceDom};
pub use uniform_interface::{
    UniformInterface, UniformInterfaceDom, UniformNonUnit, UniformUnion, UniformVisitor,
};
pub use vs_interface::{VertexField, VertexVisitor, VsInterface, VsInterfaceDom};
