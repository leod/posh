use glow::HasContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompareFunction {
    Always,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Never,
    NotEqual,
}

impl CompareFunction {
    pub const fn to_gl(self) -> u32 {
        use CompareFunction::*;

        match self {
            Always => glow::ALWAYS,
            Equal => glow::EQUAL,
            Greater => glow::GREATER,
            GreaterOrEqual => glow::GEQUAL,
            Less => glow::LESS,
            LessOrEqual => glow::LEQUAL,
            Never => glow::NEVER,
            NotEqual => glow::NOTEQUAL,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Viewport {
    pub lower_left_corner: glam::UVec2,
    pub size: glam::UVec2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CullFace {
    Front,
    Back,
}

impl CullFace {
    pub const fn to_gl(self) -> u32 {
        use CullFace::*;

        match self {
            Front => glow::FRONT,
            Back => glow::BACK,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DrawParams {
    pub clear_color: Option<glam::Vec4>,
    pub clear_depth: Option<f32>,
    pub clear_stencil: Option<u8>,
    pub depth_compare: Option<CompareFunction>,
    pub depth_mask: bool,
    pub color_mask: glam::BVec4,
    pub viewport: Option<Viewport>,
    pub cull_face: Option<CullFace>,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            clear_color: None,
            clear_depth: None,
            clear_stencil: None,
            depth_compare: None,
            depth_mask: true,
            color_mask: glam::BVec4::TRUE,
            viewport: None,
            cull_face: None,
        }
    }
}

impl DrawParams {
    pub(super) fn set_delta(
        &self,
        gl: &glow::Context,
        current: &DrawParams,
        framebuffer_size: glam::UVec2,
    ) {
        let mut clear_mask = 0;

        if let Some(c) = self.clear_color {
            unsafe { gl.clear_color(c.x, c.y, c.z, c.w) };

            clear_mask |= glow::COLOR_BUFFER_BIT;
        }

        if let Some(c) = self.clear_depth {
            unsafe { gl.clear_depth_f32(c) };

            clear_mask |= glow::DEPTH_BUFFER_BIT;
        }

        if let Some(c) = self.clear_stencil {
            unsafe { gl.clear_stencil(c as i32) };

            clear_mask |= glow::STENCIL_BUFFER_BIT;
        }

        if clear_mask > 0 {
            unsafe { gl.clear(clear_mask) };
        }

        if self.depth_compare != current.depth_compare {
            if let Some(func) = self.depth_compare {
                let func = func.to_gl();

                unsafe { gl.enable(glow::DEPTH_TEST) };
                unsafe { gl.depth_func(func) };
            } else {
                unsafe { gl.disable(glow::DEPTH_TEST) };
            }
        }

        if self.depth_mask != current.depth_mask {
            unsafe { gl.depth_mask(self.depth_mask) };
        }

        if self.color_mask != current.color_mask {
            let mask = self.color_mask;

            unsafe { gl.color_mask(mask.x, mask.y, mask.z, mask.w) };
        }

        let viewport = self.viewport.unwrap_or(Viewport {
            lower_left_corner: glam::UVec2::ZERO,
            size: framebuffer_size,
        });

        if self.cull_face != current.cull_face {
            if let Some(cull_face) = self.cull_face {
                let cull_face = cull_face.to_gl();

                unsafe { gl.enable(glow::CULL_FACE) };
                unsafe { gl.cull_face(cull_face) };
            } else {
                unsafe { gl.disable(glow::CULL_FACE) };
            }
        }

        unsafe {
            gl.viewport(
                viewport.lower_left_corner.x.try_into().unwrap(),
                viewport.lower_left_corner.y.try_into().unwrap(),
                viewport.size.x.try_into().unwrap(),
                viewport.size.y.try_into().unwrap(),
            )
        };
    }

    pub fn with_clear_color(mut self, clear_color: glam::Vec4) -> Self {
        self.clear_color = Some(clear_color);
        self
    }

    pub fn with_clear_depth(mut self, clear_depth: f32) -> Self {
        self.clear_depth = Some(clear_depth);
        self
    }

    pub fn with_clear_stencil(mut self, clear_stencil: u8) -> Self {
        self.clear_stencil = Some(clear_stencil);
        self
    }

    pub fn with_depth_compare(mut self, depth_compare: CompareFunction) -> Self {
        self.depth_compare = Some(depth_compare);
        self
    }

    pub fn with_depth_mask(mut self, depth_mask: bool) -> Self {
        self.depth_mask = depth_mask;
        self
    }

    pub fn with_color_mask(mut self, color_mask: glam::BVec4) -> Self {
        self.color_mask = color_mask;
        self
    }

    pub fn with_viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = Some(viewport);
        self
    }

    pub fn with_cull_face(mut self, cull_face: CullFace) -> Self {
        self.cull_face = Some(cull_face);
        self
    }
}
