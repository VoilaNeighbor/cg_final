#![feature(const_size_of_val)]
#![feature(const_slice_from_raw_parts)]

use business::scratch::BasicRenderer;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use glutin::{ContextBuilder, GlProfile, GlRequest};

use crate::framework::driver::App;

mod business;
mod framework;
mod utils;

fn main() {
	let event_loop = EventLoop::new();
	let window_builder = WindowBuilder::new().with_title("CG Final Project");
	let context = ContextBuilder::new()
		.with_gl(GlRequest::Latest)
		.with_gl_profile(GlProfile::Core)
		.build_windowed(window_builder, &event_loop)
		.unwrap();

	// safety: Only 1 GL Context throughout the program.
	let context = unsafe { context.make_current().unwrap() };

	App::new(event_loop, context).with_plugin(BasicRenderer::new_boxed).run();
}
