use glutin::event::WindowEvent;

use crate::Window;

pub struct WindowInfoTracker {
	pub width: u32,
	pub height: u32,
}

impl WindowInfoTracker {
	pub fn new(window: &Window) -> Self {
		Self { width: window.inner_size().width, height: window.inner_size().height }
	}

	pub fn on_window_event(&mut self, event: &WindowEvent) {
		if let WindowEvent::Resized(size) = event {
			self.width = size.width;
			self.height = size.height;
		}
	}
}
