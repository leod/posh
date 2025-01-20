use glow::HasContext;

#[derive(Default, Debug, Copy, Clone)]
pub struct ClearParams {
    pub color: Option<[f32; 4]>,
    pub depth: Option<f32>,
    pub stencil: Option<u8>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Rect {
    pub lower_left_corner: [u32; 2],
    pub size: [u32; 2],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Comparison {
    Always,
    Equal,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
    Never,
    NotEqual,
}

impl Comparison {
    pub const fn to_gl(self) -> u32 {
        use Comparison::*;

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
pub struct Blending {
    pub color_equation: BlendEquation,
    pub alpha_equation: BlendEquation,
    pub src_func_color: BlendFunc,
    pub dst_func_color: BlendFunc,
    pub src_func_alpha: BlendFunc,
    pub dst_func_alpha: BlendFunc,
    pub constant_color: [f32; 4],
}

impl Default for Blending {
    fn default() -> Self {
        Self {
            color_equation: BlendEquation::Add,
            alpha_equation: BlendEquation::Add,
            src_func_color: BlendFunc::One,
            src_func_alpha: BlendFunc::One,
            dst_func_color: BlendFunc::Zero,
            dst_func_alpha: BlendFunc::Zero,
            constant_color: [0.0; 4],
        }
    }
}

impl Blending {
    pub fn add() -> Self {
        Self::default().with_equation(BlendEquation::Add)
    }

    pub fn subtract() -> Self {
        Self::default().with_equation(BlendEquation::Subtract)
    }

    pub fn reverse_subtract() -> Self {
        Self::default().with_equation(BlendEquation::ReverseSubtract)
    }

    pub fn min() -> Self {
        Self::default().with_equation(BlendEquation::Min)
    }

    pub fn max() -> Self {
        Self::default().with_equation(BlendEquation::Max)
    }

    pub fn alpha() -> Self {
        Self::default()
            .with_src_func(BlendFunc::SrcAlpha)
            .with_dst_func(BlendFunc::OneMinusSrcAlpha)
    }

    pub fn with_color_equation(mut self, equation: BlendEquation) -> Self {
        self.color_equation = equation;
        self
    }

    pub fn with_alpha_equation(mut self, equation: BlendEquation) -> Self {
        self.alpha_equation = equation;
        self
    }

    pub fn with_equation(self, equation: BlendEquation) -> Self {
        self.with_color_equation(equation)
            .with_alpha_equation(equation)
    }

    pub fn with_src_func_color(mut self, func: BlendFunc) -> Self {
        self.src_func_color = func;
        self
    }

    pub fn with_src_func_alpha(mut self, func: BlendFunc) -> Self {
        self.src_func_alpha = func;
        self
    }

    pub fn with_src_func(self, func: BlendFunc) -> Self {
        self.with_src_func_color(func).with_src_func_alpha(func)
    }

    pub fn with_dst_func_color(mut self, func: BlendFunc) -> Self {
        self.dst_func_color = func;
        self
    }

    pub fn with_dst_func_alpha(mut self, func: BlendFunc) -> Self {
        self.dst_func_alpha = func;
        self
    }

    pub fn with_dst_func(self, func: BlendFunc) -> Self {
        self.with_dst_func_color(func).with_dst_func_alpha(func)
    }

    pub fn with_func(self, func: BlendFunc) -> Self {
        self.with_src_func(func).with_dst_func(func)
    }

    pub fn with_constant_color(mut self, color: [f32; 4]) -> Self {
        self.constant_color = color;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StencilOp {
    Keep,
    Zero,
    Replace,
    Increment,
    Decrement,
    Invert,
    IncrementWrap,
    DecrementWrap,
}

impl StencilOp {
    pub fn to_gl(self) -> u32 {
        use StencilOp::*;

        match self {
            Keep => glow::KEEP,
            Zero => glow::ZERO,
            Replace => glow::REPLACE,
            Increment => glow::INCR,
            Decrement => glow::DECR,
            Invert => glow::INVERT,
            IncrementWrap => glow::INCR_WRAP,
            DecrementWrap => glow::DECR_WRAP,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilTest {
    pub comparison: Comparison,
    pub reference: i32,
    pub mask: u32,
}

impl Default for StencilTest {
    fn default() -> Self {
        Self {
            comparison: Comparison::Always,
            reference: 0,
            mask: !0,
        }
    }
}

impl StencilTest {
    fn set(&self, gl: &glow::Context, face: u32) {
        let comparison = self.comparison.to_gl();

        unsafe { gl.stencil_func_separate(face, comparison, self.reference, self.mask) };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilOps {
    pub stencil_fail: StencilOp,
    pub depth_fail: StencilOp,
    pub depth_pass: StencilOp,
}

impl Default for StencilOps {
    fn default() -> Self {
        Self {
            stencil_fail: StencilOp::Keep,
            depth_fail: StencilOp::Keep,
            depth_pass: StencilOp::Keep,
        }
    }
}

impl StencilOps {
    fn set(&self, gl: &glow::Context, face: u32) {
        let stencil_fail = self.stencil_fail.to_gl();
        let depth_fail = self.depth_fail.to_gl();
        let depth_pass = self.depth_pass.to_gl();

        unsafe { gl.stencil_op_separate(face, stencil_fail, depth_fail, depth_pass) };
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DrawParams {
    pub viewport: Option<Rect>,
    pub cull_face: Option<CullFace>,
    pub scissor: Option<Rect>,
    pub stencil_test_front: Option<StencilTest>,
    pub stencil_test_back: Option<StencilTest>,
    pub stencil_ops_front: Option<StencilOps>,
    pub stencil_ops_back: Option<StencilOps>,
    pub depth_test: Option<Comparison>,
    pub blending: Option<Blending>,
    pub stencil_mask_front: u32,
    pub stencil_mask_back: u32,
    pub depth_mask: bool,
    pub color_mask: [bool; 4],
}

impl Default for DrawParams {
    fn default() -> Self {
        Self {
            viewport: None,
            cull_face: None,
            scissor: None,
            stencil_test_front: None,
            stencil_test_back: None,
            stencil_ops_front: None,
            stencil_ops_back: None,
            depth_test: None,
            blending: None,
            stencil_mask_front: !0,
            stencil_mask_back: !0,
            depth_mask: true,
            color_mask: [true; 4],
        }
    }
}

impl DrawParams {
    pub fn new() -> Self {
        Default::default()
    }

    pub(super) fn set_delta(
        &self,
        gl: &glow::Context,
        current: &DrawParams,
        framebuffer_size: [u32; 2],
    ) {
        if self.scissor != current.scissor {
            if let Some(scissor) = self.scissor {
                let x = scissor.lower_left_corner[0].try_into().unwrap();
                let y = scissor.lower_left_corner[1].try_into().unwrap();
                let width = scissor.size[0].try_into().unwrap();
                let height = scissor.size[1].try_into().unwrap();

                unsafe { gl.enable(glow::SCISSOR_TEST) };
                unsafe { gl.scissor(x, y, width, height) };
            } else {
                unsafe { gl.disable(glow::SCISSOR_TEST) };
            }
        }

        {
            let viewport = self.viewport.unwrap_or(Rect {
                lower_left_corner: [0; 2],
                size: framebuffer_size,
            });

            let x = viewport.lower_left_corner[0].try_into().unwrap();
            let y = viewport.lower_left_corner[1].try_into().unwrap();
            let width = viewport.size[0].try_into().unwrap();
            let height = viewport.size[1].try_into().unwrap();

            unsafe { gl.viewport(x, y, width, height) };
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

        if (self.stencil_test_front, self.stencil_test_back)
            != (current.stencil_test_front, current.stencil_test_back)
        {
            if self.stencil_test_front.is_some() || self.stencil_test_back.is_some() {
                unsafe { gl.enable(glow::STENCIL_TEST) };
                self.stencil_test_front
                    .unwrap_or_default()
                    .set(gl, glow::FRONT);
                self.stencil_test_front
                    .unwrap_or_default()
                    .set(gl, glow::BACK);
            } else {
                unsafe { gl.disable(glow::STENCIL_TEST) };
            }
        }

        if (self.stencil_ops_front, self.stencil_ops_back)
            != (current.stencil_ops_front, current.stencil_ops_back)
        {
            self.stencil_ops_front
                .unwrap_or_default()
                .set(gl, glow::FRONT);
            self.stencil_ops_back
                .unwrap_or_default()
                .set(gl, glow::BACK);
        }

        if self.depth_test != current.depth_test {
            if let Some(comparison) = self.depth_test {
                let comparison = comparison.to_gl();

                unsafe { gl.enable(glow::DEPTH_TEST) };
                unsafe { gl.depth_func(comparison) };
            } else {
                unsafe { gl.disable(glow::DEPTH_TEST) };
            }
        }

        if self.blending != current.blending {
            if let Some(blending) = self.blending {
                let color_equation = blending.color_equation.to_gl();
                let alpha_equation = blending.alpha_equation.to_gl();
                let src_func_color = blending.src_func_color.to_gl();
                let dst_func_color = blending.dst_func_color.to_gl();
                let src_func_alpha = blending.src_func_alpha.to_gl();
                let dst_func_alpha = blending.dst_func_alpha.to_gl();

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
                        blending.constant_color[0],
                        blending.constant_color[1],
                        blending.constant_color[2],
                        blending.constant_color[3],
                    )
                };
            } else {
                unsafe { gl.disable(glow::BLEND) };
            }
        }

        if self.stencil_mask_front != current.stencil_mask_front {
            unsafe { gl.stencil_mask_separate(glow::FRONT, self.stencil_mask_front) };
        }

        if self.stencil_mask_front != current.stencil_mask_back {
            unsafe { gl.stencil_mask_separate(glow::BACK, self.stencil_mask_back) };
        }

        if self.depth_mask != current.depth_mask {
            unsafe { gl.depth_mask(self.depth_mask) };
        }

        if self.color_mask != current.color_mask {
            let mask = self.color_mask;

            unsafe { gl.color_mask(mask[0], mask[1], mask[2], mask[3]) };
        }
    }

    pub fn with_viewport(mut self, viewport: Rect) -> Self {
        self.viewport = Some(viewport);
        self
    }

    pub fn with_cull_face(mut self, cull_face: CullFace) -> Self {
        self.cull_face = Some(cull_face);
        self
    }

    pub fn with_scissor(mut self, scissor: Rect) -> Self {
        self.scissor = Some(scissor);
        self
    }

    pub fn with_stencil_test_front(mut self, test: StencilTest) -> Self {
        self.stencil_test_front = Some(test);
        self
    }

    pub fn with_stencil_test_back(mut self, test: StencilTest) -> Self {
        self.stencil_test_back = Some(test);
        self
    }

    pub fn with_stencil_test(self, test: StencilTest) -> Self {
        self.with_stencil_test_front(test)
            .with_stencil_test_back(test)
    }

    pub fn with_stencil_ops_front(mut self, ops: StencilOps) -> Self {
        self.stencil_ops_front = Some(ops);
        self
    }

    pub fn with_stencil_ops_back(mut self, ops: StencilOps) -> Self {
        self.stencil_ops_back = Some(ops);
        self
    }

    pub fn with_stencil_ops(self, ops: StencilOps) -> Self {
        self.with_stencil_ops_front(ops).with_stencil_ops_back(ops)
    }

    pub fn with_depth_test(mut self, comparison: Comparison) -> Self {
        self.depth_test = Some(comparison);
        self
    }

    pub fn with_blending(mut self, blending: Blending) -> Self {
        self.blending = Some(blending);
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

    pub fn with_stencil_mask(self, mask: u32) -> Self {
        self.with_stencil_mask_front(mask)
            .with_stencil_mask_back(mask)
    }

    pub fn with_color_mask(mut self, mask: [bool; 4]) -> Self {
        self.color_mask = mask;
        self
    }

    pub fn with_depth_mask(mut self, mask: bool) -> Self {
        self.depth_mask = mask;
        self
    }
}
