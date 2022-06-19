use glow::{HasContext, COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, DEPTH_TEST};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{Api, ContextBuilder, GlProfile, GlRequest};

use crate::{GlContext, WinContext};

pub trait AppComponent {
	fn on_window_event(&mut self, event: &WindowEvent);
	unsafe fn render(&self, gl: &GlContext);
	fn update(&mut self);
}

pub struct App {
	event_loop: EventLoop<()>,
	win: WinContext,
	gl: GlContext,
}

impl App {
	pub unsafe fn new() -> Self {
		let event_loop = EventLoop::new();

		let win = ContextBuilder::new()
			.with_gl(GlRequest::Specific(Api::OpenGl, (4, 6)))
			.with_gl_profile(GlProfile::Core)
			.build_windowed(WindowBuilder::new(), &event_loop)
			.unwrap()
			.make_current()
			.unwrap();

		let gl = GlContext::from_loader_function(|s| win.get_proc_address(s));
		gl.enable(DEPTH_TEST);

		Self { event_loop, win, gl }
	}

	pub unsafe fn run(self, mut component: impl AppComponent + 'static) {
		self.event_loop.run(move |event, _, ctrl| match event {
			Event::WindowEvent { event, .. } => {
				if let WindowEvent::CloseRequested = event {
					*ctrl = ControlFlow::Exit
				} else {
					component.on_window_event(&event);
				}
			}
			Event::MainEventsCleared => {
				self.gl.clear_color(0.2, 0.2, 0.2, 1.0);
				self.gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
				component.render(&self.gl);
				component.update();
				self.win.swap_buffers().unwrap();
			}
			_ => {}
		})
	}

	pub fn window(&self) -> &Window {
		self.win.window()
	}

	pub fn gl(&self) -> &GlContext {
		&self.gl
	}
}
