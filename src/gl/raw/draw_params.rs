use glow::HasContext;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CompareFunc {
    Always,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Never,
    NotEqual,
}

impl CompareFunc {
    pub const fn to_gl(self) -> u32 {
        use CompareFunc::*;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlendEquation {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl BlendEquation {
    pub fn to_gl(self) -> u32 {
        use BlendEquation::*;

        match self {
            Add => glow::FUNC_ADD,
            Subtract => glow::FUNC_SUBTRACT,
            ReverseSubtract => glow::FUNC_REVERSE_SUBTRACT,
            Min => glow::MIN,
            Max => glow::MAX,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BlendFunc {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
}

impl BlendFunc {
    pub fn to_gl(self) -> u32 {
        use BlendFunc::*;

        match self {
            Zero => glow::ZERO,
            One => glow::ONE,
            SrcColor => glow::SRC_COLOR,
            OneMinusSrcColor => glow::ONE_MINUS_SRC_COLOR,
            DstColor => glow::DST_COLOR,
            OneMinusDstColor => glow::ONE_MINUS_DST_COLOR,
            SrcAlpha => glow::SRC_ALPHA,
            OneMinusSrcAlpha => glow::ONE_MINUS_SRC_ALPHA,
            DstAlpha => glow::DST_ALPHA,
            OneMinusDstAlpha => glow::ONE_MINUS_DST_ALPHA,
            ConstantColor => glow::CONSTANT_COLOR,
            OneMinusConstantColor => glow::ONE_MINUS_CONSTANT_COLOR,
            ConstantAlpha => glow::CONSTANT_ALPHA,
            OneMinusConstantAlpha => glow::ONE_MINUS_CONSTANT_ALPHA,
            SrcAlphaSaturate => glow::SRC_ALPHA_SATURATE,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Blend {
    pub color_equation: BlendEquation,
    pub alpha_equation: BlendEquation,
    pub src_func_color: BlendFunc,
    pub dst_func_color: BlendFunc,
    pub src_func_alpha: BlendFunc,
    pub dst_alpha_func: BlendFunc,
    pub constant_color: glam::Vec4,
}

impl Default for Blend {
    fn default() -> Self {
        Self {
            color_equation: BlendEquation::Add,
            alpha_equation: BlendEquation::Add,
            src_func_color: BlendFunc::One,
            src_func_alpha: BlendFunc::One,
            dst_func_color: BlendFunc::Zero,
            dst_alpha_func: BlendFunc::Zero,
            constant_color: glam::Vec4::ZERO,
        }
    }
}

impl Blend {
    pub fn with_color_equation(mut self, equation: BlendEquation) -> Self {
        self.color_equation = equation;
        self
    }

    pub fn with_alpha_equation(mut self, equation: BlendEquation) -> Self {
        self.alpha_equation = equation;
        self
    }

    pub fn with_equation(mut self, equation: BlendEquation) -> Self {
        self.with_color_equation(equation)
            .with_alpha_equation(equation)
    }

    pub fn with_src_func_color(mut self, func: BlendFunc) -> Self {
        self.src_func_color = func;
        self
    }

    pub fn with_src_alpha_func(mut self, func: BlendFunc) -> Self {
        self.src_func_alpha = func;
        self
    }

    pub fn with_src_func(mut self, func: BlendFunc) -> Self {
        self.with_src_func_color(func).with_src_alpha_func(func)
    }

    pub fn with_dst_func_color(mut self, func: BlendFunc) -> Self {
        self.dst_func_color = func;
        self
    }

    pub fn with_dst_alpha_func(mut self, func: BlendFunc) -> Self {
        self.src_func_alpha = func;
        self
    }

    pub fn with_dst_func(mut self, func: BlendFunc) -> Self {
        self.with_dst_func_color(func).with_dst_alpha_func(func)
    }

    pub fn with_constant_color(mut self, color: glam::Vec4) -> Self {
        self.constant_color = color;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DrawParams {
    pub clear_color: Option<glam::Vec4>,
    pub clear_depth: Option<f32>,
    pub clear_stencil: Option<u8>,
    pub color_mask: glam::BVec4,
    pub depth_mask: bool,
    pub stencil_mask_front: u32,
    pub stencil_mask_back: u32,
    pub depth_test: Option<CompareFunc>,
    pub cull_face: Option<CullFace>,
    pub blend: Option<Blend>,
    pub scissor: Option<glam::UVec4>,
    pub viewport: Option<Viewport>,
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            clear_color: None,
            clear_depth: None,
            clear_stencil: None,
            depth_mask: true,
            color_mask: glam::BVec4::TRUE,
            stencil_mask_front: !0,
            stencil_mask_back: !0,
            depth_test: None,
            cull_face: None,
            blend: None,
            scissor: None,
            viewport: None,
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

        if self.depth_mask != current.depth_mask {
            unsafe { gl.depth_mask(self.depth_mask) };
        }

        if self.color_mask != current.color_mask {
            let mask = self.color_mask;

            unsafe { gl.color_mask(mask.x, mask.y, mask.z, mask.w) };
        }

        if self.stencil_mask_front != current.stencil_mask_front {
            unsafe { gl.stencil_mask_separate(glow::FRONT, self.stencil_mask_front) };
        }

        if self.stencil_mask_front != current.stencil_mask_back {
            unsafe { gl.stencil_mask_separate(glow::BACK, self.stencil_mask_back) };
        }

        if self.depth_test != current.depth_test {
            if let Some(func) = self.depth_test {
                let func = func.to_gl();

                unsafe { gl.enable(glow::DEPTH_TEST) };
                unsafe { gl.depth_func(func) };
            } else {
                unsafe { gl.disable(glow::DEPTH_TEST) };
            }
        }

        if self.cull_face != current.cull_face {
            if let Some(cull_face) = self.cull_face {
                let cull_face = cull_face.to_gl();

                unsafe { gl.enable(glow::CULL_FACE) };
                unsafe { gl.cull_face(cull_face) };
            } else {
                unsafe { gl.disable(glow::CULL_FACE) };
            }
        }

        if self.blend != current.blend {
            if let Some(blend) = self.blend {
                let color_equation = blend.color_equation.to_gl();
                let alpha_equation = blend.alpha_equation.to_gl();
                let src_func_color = blend.src_func_color.to_gl();
                let src_func_alpha = blend.src_func_alpha.to_gl();
                let dst_func_color = blend.dst_func_color.to_gl();
                let dst_func_alpha = blend.dst_alpha_func.to_gl();

                unsafe { gl.enable(glow::BLEND) };
                unsafe { gl.blend_equation_separate(color_equation, alpha_equation) };
                unsafe {
                    gl.blend_func_separate(
                        src_func_color,
                        dst_func_color,
                        src_func_alpha,
                        dst_func_alpha,
                    )
                };
                unsafe {
                    gl.blend_color(
                        blend.constant_color.x,
                        blend.constant_color.y,
                        blend.constant_color.z,
                        blend.constant_color.w,
                    )
                };
            } else {
                unsafe { gl.disable(glow::BLEND) };
            }
        }

        if self.scissor != current.scissor {
            if let Some(scissor) = self.scissor {
                let x = scissor.x.try_into().unwrap();
                let y = scissor.y.try_into().unwrap();
                let width = scissor.z.try_into().unwrap();
                let height = scissor.w.try_into().unwrap();

                unsafe { gl.enable(glow::SCISSOR_TEST) };
                unsafe { gl.scissor(x, y, width, height) };
            } else {
                unsafe { gl.disable(glow::SCISSOR_TEST) };
            }
        }

        let viewport = self.viewport.unwrap_or(Viewport {
            lower_left_corner: glam::UVec2::ZERO,
            size: framebuffer_size,
        });

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

    pub fn with_depth_test(mut self, depth_compare: CompareFunc) -> Self {
        self.depth_test = Some(depth_compare);
        self
    }

    pub fn with_color_mask(mut self, mask: glam::BVec4) -> Self {
        self.color_mask = mask;
        self
    }

    pub fn with_depth_mask(mut self, mask: bool) -> Self {
        self.depth_mask = mask;
        self
    }

    pub fn with_stencil_mask_front(mut self, mask: u32) -> Self {
        self.stencil_mask_front = mask;
        self
    }

    pub fn with_stencil_mask_back(mut self, mask: u32) -> Self {
        self.stencil_mask_back = mask;
        self
    }

    pub fn with_stencil_mask(mut self, mask: u32) -> Self {
        self.with_stencil_mask_front(mask)
            .with_stencil_mask_back(mask)
    }

    pub fn with_cull_face(mut self, cull_face: CullFace) -> Self {
        self.cull_face = Some(cull_face);
        self
    }

    pub fn with_scissor(mut self, scissor: glam::UVec4) -> Self {
        self.scissor = Some(scissor);
        self
    }

    pub fn with_viewport(mut self, viewport: Viewport) -> Self {
        self.viewport = Some(viewport);
        self
    }
}
