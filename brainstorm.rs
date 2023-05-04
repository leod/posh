// Shared definitions

#[derive(UniformBindings)]
pub struct Camera<D: UniformBindingsDomain = Sl> {
    pub model_to_world: D::Mat4<f32>,
    pub world_to_screen: D::Mat4<f32>,
}

// Geometry pass

#[derive(VsBindings)]
pub struct ModelVertex<D: VertexDomain = Sl> {
    pub color: D::Vec3<f32>,
    pub model_pos: D::Vec3<f32>,
    pub model_normal: D::Vec3<f32>,
}

#[derive(FsBindings)]
pub struct GeometryOutput<D: FragmentDomain = Sl> {
    pub color: D::Attachment2d,
    pub world_pos: D::Attachment2d,
    pub world_normal: D::Attachment2d,
}

#[derive(Varying)]
struct GeometryVarying {
    pub color: sl::Vec3<f32>,
    pub world_pos: sl::Vec3<f32>,
    pub world_normal: sl::Vec3<f32>,
}

fn geometry_vertex_shader(camera: Camera, vertex: ModelVertex) -> VertexResult<GeometryVarying> {
    let normal_to_world = res.camera.model_to_world.to_mat3().inverse().transpose();

    let world_pos = res.camera.model_to_world * vertex.world_pos;
    let world_normal = (normal_to_world * vertex.world_normal).normalize();

    VertexResult {
        position: world_pos,
        varying: GeometryVarying {
            color: vertex.color,
            world_pos,
            world_normal,
        },
        ..Default::default()
    }
}

fn geometry_fragment_shader(
    _: impl Input,
    varying: GeometryVarying,
) -> FragmentResult<GeometryOutput> {
    let output = GeometryOutput {
        color: varying.color,
        world_pos: varying.world_pos,
        world_normal: varying.world_normal.to_vec4(),
    };

    FragmentResult {
        output,
        ..Default::default()
    }
}

// Light pass

#[derive(UniformBindings)]
pub struct LightParams<D: Domain = Sl> {
    world_pos: D::Vec3<f32>,
    color: D::Vec3<f32>,
    attenuation: D::F32,
    viewport_size: D::Vec2<f32>,
}

#[derive(Resource)]
pub struct LightResources<D: ResourceDomain = Sl> {
    color_tex: D::Sampler2d<f32>,
    world_pos_tex: D::Sampler2d<f32>,
    world_normal_tex: D::Sampler2d<f32>,
    params: D::UniformBindings<LightParams<D>>,
}

#[derive(Varying)]
struct LightVarying {
    color: Vec4<f32>,
    light_pos: Vec3<f32>,
}

fn light_vertex_shader(_: impl Input, vertex: Vec2<f32>) -> VertexResult<()> {
    VertexResult {
        position: vertex.to_vec4(),
        ..Default::default()
    }
}

fn light_fragment_shader(
    input: LightResources,
    _: (),
    vars: FragmentVars,
) -> FragmentResult<Vec4<f32>> {
    let tex_coord = vars.frag_coord.xy / input.params.viewport_size;

    let color = input.color_tex.lookup(tex_coord);
    let world_pos = input.world_pos_tex.lookup(tex_coord);
    let world_normal = input.world_normal_tex.lookup(tex_coord);

    let light_vector = input.params.world_pos - world_pos;
    let light_dist_sq = light_vector.dot(light_vector);
    let light_dist = light_dist_sq.sqrt();

    let diffuse = world_normal.dot(light_vector / light_dist).max(0.0) / input.params.attenuation;

    diffuse.le(0.0001).branch(
        FragmentResult {
            discard: true,
            ..Default::default()
        },
        FragmentResult {
            output: (input.params.color * diffuse).to_vec4(),
            ..Default::default()
        },
    )
}

#[derive(Attributes)]
struct InstancedVertex<D: AttributesDomain = Sl> {
    model: D::VsBindings<ModelVertex<D>>,
    instance: D::VsBindings<Instance<D>>,
}

// Usage

struct Game {
    camera: UniformBuffer<Camera>,
    light_params: UniformBuffer<LightParams>,
    model_data: VertexBuffer<ModelVertex>,
    screen_quad: VertexBuffer<sl::Vec2<f32>>,
    g_buffer: Framebuffer<GeometryOutput>,
    geometry_shader: Program<GeometryInput, ModelVertex, GeometryOutput>,
    light_shader: Program<LightResources, sl::Vec2<f32>, sl::Vec4<f32>>,
}

impl Game {
    pub fn new(ctx: Arc<Context>) -> Result<Self> {
        let camera = UniformBuffer::new(ctx.clone())?;
        let light_params = UniformBuffer::new(ctx.clone())?;

        let mut model_data = VertexBuffer::new(ctx.clone())?;
        model_data.push(ModelVertex::<Cpu> {
            position: [1.0, 2.0, 3.0],
            // ...
        });
        // ...

        let mut screen_quad = VertexBuffer::new(ctx.clone())?;
        // ...

        let g_buffer = Framebuffer::new(ctx.clone())?;

        let geometry_program = Program::new(
            ctx.clone(),
            geometry_vertex_shader,
            geometry_fragment_shader,
        )?;
        let light_program = Program::new(ctx.clone(), light_vertex_shader, light_fragment_shader)?;

        Ok(Self {
            camera,
            light_params,
            model_data,
            screen_quad,
            g_buffer,
            geometry_program,
            light_program,
        })
    }

    pub fn draw(&self) -> Result<Self> {
        // Geometry pass
        g_buffer.clear();

        glatic::draw(
            &self.geometry_program,
            &self.camera,
            &self.model_data,
            &g_buffer,
            &DrawParams::default(),
        )?;

        // Light pass
        self.light_program.draw(
            LightResources::<Bind> {
                color_tex: &self.g_buffer.textures().color,
                world_pos_tex: &self.g_buffer.textures().world_pos,
                world_normal_tex: &self.g_buffer.textures().world_normal,
                params: &self.light_params,
            },
            &self.screen_quad,
            ctx.back_buffer(),
            &DrawParams::default(),
        )
    }
}
