use posh::{
    sl::{self, Value},
    Numeric,
};

#[derive(Value)]
struct Foo<T: Numeric> {
    x: sl::Scalar<T>,
}

fn main() {}
