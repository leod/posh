use std::{rc::Rc, time::Duration};

use super::{context::ContextShared, disjoint_timer_query::DisjointTimerQuery};

#[derive(Debug, Clone, Default)]
pub struct TracingConfig {
    // TODO: Extend `TracingConfig` with configuration, such as whether
    // individual draw calls should be recorded.
}

#[derive(Debug, Clone)]
pub struct FrameTrace {
    pub duration: Duration,
}

pub(super) struct Tracing {
    last_query: Option<DisjointTimerQuery>,
    current_query: Option<DisjointTimerQuery>,
}

impl Tracing {
    pub fn new(_: TracingConfig) -> Self {
        Self {
            last_query: None,
            current_query: None,
        }
    }

    pub fn start_frame(&mut self, ctx: &Rc<ContextShared>) -> Option<FrameTrace> {
        let last_query = self.last_query.take_if(|query| query.available(ctx));

        if self.last_query.is_none() {
            assert!(
                self.current_query.is_none(),
                "`stop_frame()` must be called between successive calls of `start_frame()`",
            );

            self.current_query = DisjointTimerQuery::new(ctx).ok();

            if let Some(query) = self.current_query.as_ref() {
                query.start(ctx);
            }
        }

        last_query
            .and_then(|q| q.get(ctx))
            .map(|duration| FrameTrace { duration })
    }

    pub fn stop_frame(&mut self, ctx: &ContextShared) {
        if let Some(query) = self.current_query.as_ref() {
            query.stop(ctx);

            self.last_query = self.current_query.take();
        }
    }
}
