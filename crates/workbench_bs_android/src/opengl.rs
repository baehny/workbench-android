use glow::{self as GL, HasContext};
use glutin::{
    config::{Config, ConfigSurfaceTypes, ConfigTemplate, ConfigTemplateBuilder, GlConfig},
    context::*,
    display::{Display, DisplayApiPreference},
    prelude::GlDisplay,
    surface::{Surface, SurfaceAttributes, SurfaceAttributesBuilder, WindowSurface},
};
use log::{self as logging, info};
use std::ffi::CString;
use std::num::NonZeroU32;
use winit::raw_window_handle::{HasWindowHandle, RawDisplayHandle};
use winit::{raw_window_handle::RawWindowHandle, window::Window};

pub fn initialize_context(
    wgl_window: &Window,
    wgl_window_handle: RawWindowHandle,
    gl_display: &Display,
) -> (
    PossiblyCurrentContext,
    glow::Context,
    Option<Surface<WindowSurface>>,
    Config,
) {
    // Create the config we'll be used for window. We'll use the native window
    // raw-window-handle for it to get the right visual and use proper hdc. Note
    // that you can likely use it for other windows using the same config.
    let template = config_template(Some(wgl_window_handle));

    let config = unsafe { gl_display.find_configs(template) }
        .unwrap()
        .reduce(|accum, config| {
            // Find the config with the maximum number of samples.
            //
            // In general if you're not sure what you want in template you can request or
            // don't want to require multisampling for example, you can search for a
            // specific option you want afterwards.
            //
            // XXX however on macOS you can request only one config, so you should do
            // a search with the help of `find_configs` and adjusting your template.

            // Since we try to show off transparency try to pick the config that supports it
            // on X11 over the ones without it. XXX Configs that support
            // transparency on X11 tend to not have multisapmling, so be aware
            // of that.

            // #[cfg(x11_platform)]
            // let transparency_check = config
            //     .x11_visual()
            //     .map(|v| v.supports_transparency())
            //     .unwrap_or(false)
            //     & !accum
            //         .x11_visual()
            //         .map(|v| v.supports_transparency())
            //         .unwrap_or(false);

            // #[cfg(not(x11_platform))]
            let transparency_check = false;

            if transparency_check
                || config.num_samples() < accum.num_samples()
                || config.depth_size() < accum.depth_size()
                || config.stencil_size() < accum.stencil_size()
            {
                config
            } else {
                accum
            }
        })
        .unwrap();

    #[cfg(debug_assertions)]
    let with_debug = true;

    #[cfg(not(debug_assertions))]
    let with_debug = false;

    // The context creation part. It can be created before surface and that's how
    // it's expected in multithreaded + multiwindow operation mode, since you
    // can send NotCurrentContext, but not Surface.
    let context_attributes = ContextAttributesBuilder::new()
        .with_debug(with_debug)
        // .with_profile(glutin::context::GlProfile::Compatibility)
        // .with_profile(glutin::context::GlProfile::Core)
        .build(Some(wgl_window_handle));

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(Some(wgl_window_handle));

    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    });

    let attrs = surface_attributes(wgl_window);
    let surface = Some(unsafe { gl_display.create_window_surface(&config, &attrs).unwrap() });

    let current_context = not_current_gl_context
        .take()
        .unwrap()
        .make_current(surface.as_ref().unwrap())
        .unwrap();

    if !current_context.is_current() {
        panic!();
    }

    let mut gl = unsafe {
        glow::Context::from_loader_function(|symbol| {
            let symbol = CString::new(symbol).unwrap();

            gl_display.get_proc_address(symbol.as_c_str()) as *const _
        })
    };

    gl::load_with(|symbol| {
        let symbol = CString::new(symbol).unwrap();

        gl_display.get_proc_address(symbol.as_c_str()) as *const _
    });

    //
    // Enable debugging
    //
    if with_debug && gl.supports_debug() {
        logging::info!("Context supports debugging.");

        unsafe {
            gl.debug_message_callback(|_source, message_type, _id, severity, message| {
                match severity {
                    GL::DEBUG_SEVERITY_NOTIFICATION => return,
                    _ => {}
                }

                match message_type {
                    GL::DEBUG_TYPE_ERROR => logging::error!("{}", message),
                    GL::DEBUG_TYPE_UNDEFINED_BEHAVIOR => logging::error!("{}", message),
                    GL::DEBUG_TYPE_DEPRECATED_BEHAVIOR => logging::warn!("{}", message),
                    GL::DEBUG_TYPE_PERFORMANCE => logging::warn!("{}", message),

                    GL::DEBUG_TYPE_PUSH_GROUP => {}
                    GL::DEBUG_TYPE_POP_GROUP => {}

                    _ => {
                        logging::info!("{}", message)
                    }
                }
            });
        }
    }

    //
    // Report capabilities
    //
    let version = gl.version();
    logging::info!("version: {}.{}", version.major, version.minor);
    logging::info!("vendor: {}", version.vendor_info);

    let extensions = gl.supported_extensions();
    let mut available_extensions: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();

    available_extensions.sort();

    for extension in available_extensions {
        info!("{}", extension);
    }

    (current_context, gl, surface, config)
}

/// Create template to find OpenGL config.
pub fn config_template(raw_window_handle: Option<RawWindowHandle>) -> ConfigTemplate {
    let mut builder = ConfigTemplateBuilder::new().with_alpha_size(8);

    if let Some(raw_window_handle) = raw_window_handle {
        builder = builder
            .compatible_with_native_window(raw_window_handle)
            .with_surface_type(ConfigSurfaceTypes::WINDOW);
    }

    // #[cfg(cgl_backend)]
    // let builder = builder.with_transparency(true).with_multisampling(8);

    builder.build()
}

/// Create surface attributes for window surface.
pub fn surface_attributes(window: &Window) -> SurfaceAttributes<WindowSurface> {
    let (width, height): (u32, u32) = window.inner_size().into();
    let raw_window_handle = window
        .window_handle()
        .expect("Failed to get raw window handle")
        .as_raw();

    SurfaceAttributesBuilder::<WindowSurface>::new().build(
        raw_window_handle,
        NonZeroU32::new(width).unwrap(),
        NonZeroU32::new(height).unwrap(),
    )
}

pub fn create_display(
    raw_display: RawDisplayHandle,
    raw_window_handle: Option<RawWindowHandle>,
) -> Display {
    let preference = DisplayApiPreference::Egl;

    // Create connection to underlying OpenGL client Api.
    // Caution: glutin::Display::new(...) might change the current OpenGL context.
    //          This is probably caused by the construction of an internal dummy window.
    unsafe { Display::new(raw_display, preference).unwrap() }
}
