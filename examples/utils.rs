use posh::gl;

#[cfg(target_family = "wasm")]
pub fn run_demo<Demo: 'static>(
    demo_name: &str,
    demo_new: fn(gl::Context) -> Result<Demo, gl::CreateError>,
    demo_draw: fn(&mut Demo) -> Result<(), gl::DrawError>,
) {
    use wasm_bindgen::JsCast;
    use winit::{
        dpi::LogicalSize,
        event::{Event, WindowEvent},
        event_loop::EventLoop,
        platform::web::WindowExtWebSys,
        window::WindowBuilder,
    };

    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title(&format!("posh demo: {demo_name}"))
        .with_inner_size(LogicalSize::new(400.0, 200.0))
        .build(&event_loop)
        .unwrap();

    let canvas = window.canvas();

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&canvas)
        .unwrap();

    let webgl2_context = window
        .canvas()
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    let gl = glow::Context::from_webgl2_context(webgl2_context);
    let gl = gl::Context::new(gl).unwrap();

    let mut demo = demo_new(gl).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            window_id,
        } if window_id == window.id() => control_flow.set_exit(),
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::RedrawRequested(_) => {
            demo_draw(&mut demo).unwrap();
        }
        _ => (),
    });
}

#[cfg(not(target_family = "wasm"))]
pub fn run_demo<Demo: 'static>(
    demo_name: &str,
    demo_new: fn(gl::Context) -> Result<Demo, gl::CreateError>,
    demo_draw: fn(&mut Demo) -> Result<(), gl::DrawError>,
) {
    simple_logger::init().unwrap();

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video
        .window(&format!("posh demo: {demo_name}"), 1024, 768)
        .opengl()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    let gl = gl::Context::new(gl).unwrap();
    let mut demo = demo_new(gl).unwrap();

    let mut event_loop = sdl.event_pump().unwrap();

    loop {
        for event in event_loop.poll_iter() {
            use sdl2::event::Event::*;

            match event {
                Quit { .. } => return,
                _ => {}
            }
        }

        demo_draw(&mut demo).unwrap();
        window.gl_swap_window();
    }
}

#[cfg(target_family = "wasm")]
pub async fn init_wasm() {
    use wasm_bindgen_futures::JsFuture;

    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");

    // Orientation locking seems to be a new feature, so this might not work on
    // many (any?) devices.
    match web_sys::window()
        .unwrap()
        .screen()
        .unwrap()
        .orientation()
        .lock(web_sys::OrientationLockType::Landscape)
    {
        Ok(promise) => {
            let result = JsFuture::from(promise).await;
            if let Err(error) = result {
                log::info!("Failed to lock orientation: {:?}", error);
            }
        }
        Err(error) => {
            log::info!("Failed to create lock promise: {:?}", error);
        }
    }
}
