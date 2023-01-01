use std::rc::Rc;

pub(crate) struct ContextShared {}

pub struct Context {
    shared: Rc<ContextShared>,
    gl: Rc<glow::Context>,
}

impl Context {
    pub fn gl(&self) -> &Rc<glow::Context> {
        &self.gl
    }

    pub(crate) fn shared(&self) -> &Rc<ContextShared> {
        &self.shared
    }
}
