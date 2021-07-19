use wasm_bindgen::prelude::*;
use web_sys::console;

use lib::raytracer::*;
use lib::object::*;
use lib::math::*;

use winit::window::{WindowBuilder};
use winit::platform::web::{WindowExtWebSys};
use winit::dpi::{LogicalPosition, LogicalSize, PhysicalSize};
use winit::event_loop::{EventLoop, ControlFlow};
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit_input_helper::WinitInputHelper;

const SCREEN_WIDTH: u32 = 640;
const SCREEN_HEIGHT: u32 = 480;

#[wasm_bindgen]
pub fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
	.with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
	let canvas = window.canvas();
    });
}
