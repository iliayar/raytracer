use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::Clamped;

use lib::raytracer::*;
use lib::object::*;
use lib::math::{Vec3, CameraTransform};

use winit::window::{WindowBuilder};
use winit::platform::web::{WindowExtWebSys, WindowBuilderExtWebSys};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

#[wasm_bindgen]
pub fn main(canvas: web_sys::HtmlCanvasElement) {
    // let document = web_sys::window().unwrap().document().unwrap();
    // let canvas = document
    // 	.create_element("canvas").unwrap()
    // 	.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    // document.body().unwrap().append_child(&canvas).unwrap();
    canvas.set_width(SCREEN_WIDTH);
    canvas.set_height(SCREEN_HEIGHT);
    canvas.style().set_property("border", "solid").unwrap();
    let context = canvas
	.get_context("2d").unwrap()
	.unwrap()
	.dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
	.with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with_canvas(Some(canvas))
        .build(&event_loop)
        .unwrap();

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
    scene.add_light(AmbientLight::new(0.05));
    let mut raytracer = Raytracer::new(scene);

    let movement_keymap: Vec<(VirtualKeyCode, CameraTransform)> = vec![
	(VirtualKeyCode::W, CameraTransform::Move(1.)),
	(VirtualKeyCode::S, CameraTransform::Move(-1.)),
	(VirtualKeyCode::Left, CameraTransform::RotateHorizontal(std::f64::consts::FRAC_PI_8)),
	(VirtualKeyCode::Right, CameraTransform::RotateHorizontal(-std::f64::consts::FRAC_PI_8)),
	(VirtualKeyCode::Up, CameraTransform::RotateVertical(-std::f64::consts::FRAC_PI_8)),
	(VirtualKeyCode::Down, CameraTransform::RotateVertical(std::f64::consts::FRAC_PI_8)),
	(VirtualKeyCode::Plus, CameraTransform::ScaleDistance(2.)),
	(VirtualKeyCode::Plus, CameraTransform::ScaleDistance(0.5)),
    ];

    event_loop.run(move |event, _, flow_control| {
	if let Event::RedrawRequested(_) = event {
	    let rt_canvas: &Canvas = raytracer.render();
	    let mut rt_data: Vec<u8> = rt_canvas.iter().cloned().flat_map(|Pixel(r, g, b)| vec![r, g, b, 0xff].into_iter()).collect();
	    let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut rt_data), SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
	    context.put_image_data(&data, 0.0, 0.0);
	}

	if input.update(&event) {
	    for (key, mv) in movement_keymap.iter() {
		if input.key_pressed(*key) {
		    raytracer.scene.camera.transform(*mv);
		    window.request_redraw();
		}
	    }
	}
    });
}
