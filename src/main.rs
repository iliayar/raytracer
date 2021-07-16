mod lib;

use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::WindowEvent::CloseRequested;
use winit::event_loop::ControlFlow::Exit;
use winit::event_loop::EventLoop;
use winit::event::Event;
use winit_input_helper::WinitInputHelper;
use pixels::{SurfaceTexture, Pixels};

use lib::raytracer::*;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, width, height, _) = create_window("Test pixels", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap();

    let mut raytracer = Raytracer::new();
    let scene = Scene::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    event_loop.run(move |event, _, flow_control| {
	if let Event::WindowEvent { event: CloseRequested, .. } = event {
	    *flow_control = Exit;
	}
	if let Event::RedrawRequested(_) = event {
	    let frame = pixels.get_frame();
	    let canvas: &Canvas = raytracer.render(&scene);
	    for (pixel, Pixel(r, g, b)) in frame.chunks_exact_mut(4).zip(canvas.iter().cloned()) {
		    pixel[0] = r;
		    pixel[1] = g;
		    pixel[2] = b;
		    pixel[3] = 0xff;
	    }
	    pixels.render().unwrap();
	}

	if input.update(&event) {
	    if let Some(size) = input.window_resized() {
		pixels.resize_surface(size.width, size.height);
		window.request_redraw();
	    }
	}
    });
}

/// Create a window for the game.
///
/// Automatically scales the window to cover about 2/3 of the monitor height.
///
/// # Returns
///
/// Tuple of `(window, surface, width, height, hidpi_factor)`
/// `width` and `height` are in `PhysicalSize` units.
fn create_window(
    title: &str,
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, u32, u32, f64) {
    // Create a hidden window so we can estimate a good default window size
    let window = winit::window::WindowBuilder::new()
        .with_visible(false)
        .with_title(title)
        .build(event_loop)
        .unwrap();
    let hidpi_factor = window.scale_factor();

    // Get dimensions
    let width = SCREEN_WIDTH as f64;
    let height = SCREEN_HEIGHT as f64;
    let (monitor_width, monitor_height) = {
        if let Some(monitor) = window.current_monitor() {
            let size = monitor.size().to_logical(hidpi_factor);
            (size.width, size.height)
        } else {
            (width, height)
        }
    };
    let scale = (monitor_height / height * 2.0 / 3.0).round().max(1.0);

    // Resize, center, and display the window
    let min_size: winit::dpi::LogicalSize<f64> =
        PhysicalSize::new(width, height).to_logical(hidpi_factor);
    let default_size = LogicalSize::new(width * scale, height * scale);
    let center = LogicalPosition::new(
        (monitor_width - width * scale) / 2.0,
        (monitor_height - height * scale) / 2.0,
    );
    window.set_maximized(false);
    window.set_inner_size(default_size);
    window.set_min_inner_size(Some(min_size));
    window.set_outer_position(center);
    window.set_resizable(false);
    window.set_visible(true);

    let size = default_size.to_physical::<f64>(hidpi_factor);

    (
        window,
        size.width.round() as u32,
        size.height.round() as u32,
        hidpi_factor,
    )
}
