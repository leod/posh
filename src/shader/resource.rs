pub trait Resource {
    fn func_arg() -> Self;
}

pub trait UniformBlock: Resource {}

pub trait Resources {
    fn func_arg() -> Self;
}

impl<D> Resources for D
where
    D: Resource,
{
    fn func_arg() -> Self {
        <Self as Resource>::func_arg()
    }
}
