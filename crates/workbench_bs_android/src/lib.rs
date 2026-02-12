use log::info;

use winit::application::ApplicationHandler;
use winit::event::{DeviceEvent, DeviceId, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::platform::android::EventLoopBuilderExtAndroid;
use winit::window::{Window, WindowId};
use winit::platform::android::activity::AndroidApp;

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    info!("Rust android_main");

    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info)
    );

    let event_loop = EventLoop::builder()
        .with_android_app(app)
        .build()
        .unwrap();

    let mut app_state = MyApp::default();

    let _ = event_loop.run_app(&mut app_state);
}

#[derive(Default)]
struct MyApp {
    window: Option<Window>,
}

impl ApplicationHandler for MyApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            info!("Create window");

            let win_attrs = Window::default_attributes().with_title("Rust Winit Window");

            let window = event_loop.create_window(win_attrs).unwrap();

            self.window = Some(window);
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
            _ => (),
        }
    }
}
