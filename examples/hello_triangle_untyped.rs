use glow::HasContext;
use posh::gl::{untyped, GeometryType};

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window("Hello triangle!", 1024, 769)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };

    let mut event_loop = sdl.event_pump().unwrap();

    let context = untyped::Context::new(gl);

    let vertex_array = context
        .create_vertex_array(&[], None)
        .expect("Cannot create vertex array");

    let program_def = untyped::ProgramDef {
        vertex_shader_source: r#"
            #version 330
            const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }
        "#
        .to_string(),
        fragment_shader_source: r#"
            #version 330
            precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }
        "#
        .to_string(),
        ..Default::default()
    };

    let program = context
        .create_program(program_def)
        .expect("Cannot create program");

    unsafe {
        context.gl().clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let mut running = true;
    while running {
        for event in event_loop.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => running = false,
                _ => {}
            }
        }

        unsafe {
            context.gl().clear(glow::COLOR_BUFFER_BIT);
            program.draw(&[], vertex_array.stream(0..3, GeometryType::Triangles));
        }

        window.gl_swap_window();
    }
}
