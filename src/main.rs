mod lib;

use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event::WindowEvent::CloseRequested;
use winit::event_loop::ControlFlow::Exit;
use winit::event_loop::EventLoop;
use winit::event::{Event, VirtualKeyCode};
use winit_input_helper::WinitInputHelper;
use pixels::{SurfaceTexture, Pixels};

use lib::raytracer::*;
use lib::object::*;
use lib::math::{Vec3, CameraTransform};

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let (window, width, height, _) = create_window("Test pixels", &event_loop);
    let surface_texture = SurfaceTexture::new(width, height, &window);
    let mut pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap();

    let mut scene = Scene::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    scene.add(Polygon::new(
	Vec3(-1., 0., 0.),
	Vec3(1., 0., 0.),
	Vec3(-1., 0., 1.),
	Material::new(Color::new(0x00, 0xff, 0x00), 0.8)
    ));
    scene.add(Polygon::new(
	Vec3(1., 0., 0.),
	Vec3(-1., 0., 1.),
	Vec3(1., 0., 1.),
	Material::new(Color::new(0xff, 0x00, 0x00), 0.3)
    ));
    scene.add(Plane::new(
	Vec3(0., -1., 0.), 0.,
	Material::new(Color::new(0x50, 0x50, 0x50), 0.5)
    ));
    scene.add(Sphere::new(
	Vec3(0., 0., 0.5), 0.5,
	Material::new_shine(Color::new(0x00, 0x00, 0xff), 8, 0.6)
    ));
    scene.add(Sphere::new(
	Vec3(-1.5, 0.2, 0.5), 0.2,
	Material::new(Color::new(0xff, 0xff, 0x00), 0.1)
    ));
    scene.add(Sphere::new(
	Vec3(1.50, 0.2, 0.5), 0.2,
	Material::new_shine(Color::new(0xff, 0xff, 0x00), 10, 0.8)
    ));
    scene.add_light(PointLight::new(Vec3(0., 0.5, 2.), 0.75));
    scene.add_light(DirectLight::new(Vec3(0., -1., 1.), 0.2));
    // scene.add_light(PointLight::new_color(Vec3(-1.5, 3., 2.), 0.4, Color::new(0xff, 0x00, 0x00)));
    scene.add_light(AmbientLight::new(0.05));
    // scene.camera.transform(Transform::ScaleCameraScreen(3.));
    // scene.camera.transform(Transform::ScaleCameraDistance(10.));
    // scene.camera.transform(Transform::MoveCamera(-20.));
    let mut raytracer = Raytracer::new(scene);

    event_loop.run(move |event, _, flow_control| {
	if let Event::WindowEvent { event: CloseRequested, .. } = event {
	    *flow_control = Exit;
	}
	if let Event::RedrawRequested(_) = event {
	    let frame = pixels.get_frame();
	    let canvas: &Canvas = raytracer.render();
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
	    if input.key_pressed(VirtualKeyCode::W) {
		raytracer.scene.camera.transform(CameraTransform::Move(1.));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::S) {
		raytracer.scene.camera.transform(CameraTransform::Move(-1.));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::Left) {
		raytracer.scene.camera.transform(CameraTransform::RotateHorizontal(std::f64::consts::FRAC_PI_8));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::Right) {
		raytracer.scene.camera.transform(CameraTransform::RotateHorizontal(-std::f64::consts::FRAC_PI_8));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::Up) {
		raytracer.scene.camera.transform(CameraTransform::RotateVertical(-std::f64::consts::FRAC_PI_8));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::Down) {
		raytracer.scene.camera.transform(CameraTransform::RotateVertical(std::f64::consts::FRAC_PI_8));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::Plus) {
		raytracer.scene.camera.transform(CameraTransform::ScaleDistance(2.));
		window.request_redraw()
	    }
	    if input.key_pressed(VirtualKeyCode::Minus) {
		raytracer.scene.camera.transform(CameraTransform::ScaleDistance(0.5));
		window.request_redraw()
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
