use std::{
    rc::{Rc, Weak},
    time::Duration,
};

use glow::HasContext;

use super::{context::ContextShared, error::QueryError};

pub(super) struct DisjointTimerQuery {
    // `ctx` is a weak pointer to prevent a cycle between `ContextShared` and
    // `Tracing`.
    ctx: Weak<ContextShared>,
    id: glow::Query,
}

impl DisjointTimerQuery {
    pub fn new(ctx: &Rc<ContextShared>) -> Result<Self, QueryError> {
        if !ctx.caps().disjoint_timer_query_webgl2 {
            return Err(QueryError::Unsupported);
        }

        let gl = ctx.gl();
        let id = unsafe { gl.create_query() }.map_err(QueryError::ObjectCreation)?;

        #[cfg(debug_assertions)]
        super::error::check_gl_error(gl, "after query creation").map_err(QueryError::Unexpected)?;

        Ok(Self {
            ctx: Rc::downgrade(ctx),
            id,
        })
    }

    pub fn start(&self, ctx: &ContextShared) {
        let gl = ctx.gl();

        unsafe { gl.begin_query(glow::TIME_ELAPSED, self.id) };
    }

    pub fn stop(&self, ctx: &ContextShared) {
        let gl = ctx.gl();

        unsafe { gl.end_query(glow::TIME_ELAPSED) };
    }

    pub fn available(&self, ctx: &ContextShared) -> bool {
        let gl = ctx.gl();

        let available =
            unsafe { gl.get_query_parameter_u32(self.id, glow::QUERY_RESULT_AVAILABLE) };

        available != 0
    }

    pub fn get(&self, ctx: &ContextShared) -> Option<Duration> {
        let gl = ctx.gl();

        // TODO: Use `GPU_DISJOINT` constant from `glow` once available.
        let disjoint = unsafe { gl.get_parameter_bool(0x8FBB) };

        if self.available(ctx) && !disjoint {
            // FIXME: The spec
            // (<https://registry.khronos.org/webgl/extensions/EXT_disjoint_timer_query_webgl2/>)
            // seems to say that the query result should be an `u64`, which
            // makes sense, but I'm not sure how to retrieve an `u64` with
            // `glow`.
            let nanos = unsafe { gl.get_query_parameter_u32(self.id, glow::QUERY_RESULT) };

            Some(Duration::from_nanos(nanos as u64))
        } else {
            None
        }
    }
}

impl Drop for DisjointTimerQuery {
    fn drop(&mut self) {
        if let Some(ctx) = self.ctx.upgrade() {
            let gl = ctx.gl();

            unsafe { gl.delete_query(self.id) };
        }
    }
}
