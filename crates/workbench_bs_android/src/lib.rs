use glow::{Context, HasContext};
use glutin::context::PossiblyCurrentContext;
use glutin::surface::{GlSurface, Surface, WindowSurface};
use glow::{self as GL};

use log::info;

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::android::EventLoopBuilderExtAndroid;
use winit::platform::android::activity::AndroidApp;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use winit::window::{Window, WindowId};

pub mod opengl;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    info!("Rust android_main");

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    let event_loop = EventLoop::builder().with_android_app(app).build().unwrap();

    let mut app_state = MyApp::default();

    let _ = event_loop.run_app(&mut app_state);
}

#[derive(Default)]
struct MyApp {
    window: Option<Window>,
    gl_context: Option<Context>,
    opengl: Option<PossiblyCurrentContext>,
    surface: Option<Surface<WindowSurface>>,
}

impl ApplicationHandler for MyApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            info!("Create window");

            let win_attrs = Window::default_attributes().with_title("Rust Winit Window");

            let window = event_loop.create_window(win_attrs).unwrap();

            let raw_display = event_loop
                .display_handle()
                .expect("Failed to get raw display handle")
                .as_raw();

            let raw_window_handle = window
                .window_handle()
                .expect("Should get the Window Handle required for the WGL initialization")
                .into();

            let gl_display = opengl::create_display(raw_display, Some(raw_window_handle));

            let (current_context, gl_context, surface, config) =
                opengl::initialize_context(&window, window.window_handle().unwrap().into(), &gl_display);

            self.window = Some(window);
            self.gl_context = Some(gl_context);
            self.opengl = Some(current_context);
            self.surface = surface;
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        info!("{:?}", event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        info!("{:?}", event);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested => {
                if let Some(window) = self.window.as_ref() {
                    if let Some(gl) = self.gl_context.as_mut() {
                        unsafe {
                            gl.clear_color(1.0, 0.0, 0.0, 1.0);
                            gl.clear(GL::COLOR_BUFFER_BIT);

                            if let Some(surface) = self.surface.as_mut() {
                                // Calling swap_buffers is necessary to have touch events!
                                surface.swap_buffers(self.opengl.as_mut().unwrap()).unwrap();
                            }
                        }
                    }

                    // window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
