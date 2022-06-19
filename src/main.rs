#![feature(const_size_of_val)]
#![feature(const_slice_from_raw_parts)]
#![feature(default_free_fn)]

use std::default::default;
use std::time::Instant;

use glutin::event::{DeviceEvent, WindowEvent};
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use nalgebra::Point3;

use crate::framework::app::{App, AppComponent};
use crate::graphics::camera::Camera;
use crate::graphics::renderer::Renderer;
use crate::interaction::camera_controller::CameraController;
use crate::misc::clock::Clock;
use crate::misc::window_info_tracker::WindowInfoTracker;

mod framework;
mod graphics;
mod misc;
mod interaction;

pub type GlContext = glow::Context;
pub type WinContext = ContextWrapper<PossiblyCurrent, Window>;

struct MainComponents {
	renderer: Renderer,
	wit: WindowInfoTracker,
	cc: CameraController,
	clock: Clock,
}

impl MainComponents {
	pub fn new(app: &App) -> Self {
		Self {
			renderer: Renderer::new(app.gl()),
			wit: WindowInfoTracker::new(app.window()),
			cc: default(),
			clock: Clock::default(),
		}
	}
}

impl AppComponent for MainComponents {
	fn on_window_event(&mut self, event: &WindowEvent) {
		self.wit.on_window_event(event);
		self.cc.on_window_event(event);
	}

	fn on_device_event(&mut self, event: &DeviceEvent) {
		self.cc.on_device_event(event, &self.clock);
	}

	unsafe fn render(&self, gl: &GlContext) {
		self.renderer.render(gl, &self.wit, self.cc.camera(), &self.clock);
	}

	fn update(&mut self) {
		self.clock.update();
	}
}

fn main() {
	unsafe {
		let app = App::new();
		let main_component = MainComponents::new(&app);
		app.run(main_component);
	}
}
