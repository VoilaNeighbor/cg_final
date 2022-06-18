pub use glow::Context as GlContext;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::Window;
use glutin::{ContextWrapper as WinContext, PossiblyCurrent};

pub trait Plugin {
	fn render(&self, gl: &GlContext);
}

pub struct App {
	event_loop: EventLoop<()>,
	gl_ctx: GlContext,
	win_ctx: WinContext<PossiblyCurrent, Window>,
	plugins: Vec<Box<dyn Plugin>>,
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
