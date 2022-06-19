#![feature(const_size_of_val)]
#![feature(const_slice_from_raw_parts)]
#![feature(default_free_fn)]

use std::default::default;

use glutin::event::WindowEvent;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};

use crate::framework::app::{App, AppComponent};
use crate::graphics::camera::Camera;
use crate::graphics::renderer::Renderer;
use crate::misc::window_info_tracker::WindowInfoTracker;

mod framework;
mod graphics;
mod misc;

pub type GlContext = glow::Context;
pub type WinContext = ContextWrapper<PossiblyCurrent, Window>;

struct MainComponents {
	renderer: Renderer,
	window_info_tracker: WindowInfoTracker,
	camera: Camera,
}

impl MainComponents {
	pub fn new(app: &App) -> Self {
		Self {
			renderer: Renderer::new(app.gl()),
			window_info_tracker: WindowInfoTracker::new(app.window()),
			camera: default(),
		}
	}
}

impl AppComponent for MainComponents {
	fn on_window_event(&mut self, event: &WindowEvent) {
		self.window_info_tracker.on_window_event(event);
	}

	unsafe fn render(&self, gl: &GlContext) {
		self.renderer.render(gl, &self.window_info_tracker, &self.camera);
	}
}

fn main() {
	unsafe {
		let app = App::new();
		let main_component = MainComponents::new(&app);
		app.run(main_component);
	}
}
