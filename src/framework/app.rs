pub use glow::Context as GlContext;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::{Window, WindowBuilder};
use glutin::{ContextBuilder, ContextWrapper as WinContext, GlProfile, GlRequest, PossiblyCurrent};

pub trait Plugin {
	fn render(&self, gl: &GlContext);
}

pub struct App {
	event_loop: EventLoop<()>,
	gl_ctx: GlContext,
	win_ctx: WinContext<PossiblyCurrent, Window>,
	plugins: Vec<Box<dyn Plugin>>,
}

impl Default for App {
	fn default() -> Self {
		let event_loop = EventLoop::new();
		let window_builder = WindowBuilder::new().with_title("CG Final Project");
		let context = ContextBuilder::new()
			.with_gl(GlRequest::Latest)
			.with_gl_profile(GlProfile::Core)
			.build_windowed(window_builder, &event_loop)
			.unwrap();

		// safety: Only 1 GL Context throughout the program.
		let context = unsafe { context.make_current().unwrap() };

		Self::new(event_loop, context)
	}
}

impl App {
	pub fn new(event_loop: EventLoop<()>, win_ctx: WinContext<PossiblyCurrent, Window>) -> Self {
		Self {
			event_loop,
			// safety: Well, GL is inherently unsafe. :P
			gl_ctx: unsafe { GlContext::from_loader_function(|s| win_ctx.get_proc_address(s)) },
			win_ctx,
			plugins: Vec::new(),
		}
	}

	pub fn with_plugin<F>(mut self, builder: F) -> Self
	where
		F: FnOnce(&GlContext) -> Box<dyn Plugin>,
	{
		self.plugins.push(builder(&mut self.gl_ctx));
		self
	}

	pub fn run(self) {
		self.event_loop.run(move |event, _, ctrl| match event {
			Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => *ctrl = ControlFlow::Exit,
			Event::RedrawRequested(_) => {
				for p in &self.plugins {
					p.render(&self.gl_ctx);
				}
				self.win_ctx.swap_buffers().unwrap();
			}
			_ => {}
		})
	}
}
