use std::mem::size_of;

use glow::{
	HasContext, NativeBuffer, NativeProgram, NativeVertexArray, ARRAY_BUFFER, DYNAMIC_DRAW, FLOAT, FRAGMENT_SHADER, TRIANGLES, VERTEX_SHADER
};

use crate::framework::driver::{GlContext, Plugin};
use crate::utils::as_bytes;

#[repr(C, packed)]
struct Vertex {
	position: [f32; 2],
}

const VERTICES: [Vertex; 3] = [
	Vertex { position: [0.0, 0.5] },
	Vertex { position: [0.5, -0.5] },
	Vertex { position: [-0.5, -0.5] },
];

pub struct BasicRenderer {
	vao: NativeVertexArray,
	program: NativeProgram,
	_vbo: NativeBuffer,
}

impl BasicRenderer {
	pub fn new_boxed(gl: &GlContext) -> Box<dyn Plugin> {
		unsafe {
			let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
			gl.shader_source(
				vert_shader,
				r#"#version 460 core
				layout(location = 0) in vec2 position;
				void main() {
					gl_Position = vec4(position, 0.0, 1.0);
				}"#,
			);
			gl.compile_shader(vert_shader);
			if !gl.get_shader_compile_status(vert_shader) {
				panic!("{}", gl.get_shader_info_log(vert_shader));
			}

			let frag_shader = gl.create_shader(FRAGMENT_SHADER).unwrap();
			gl.shader_source(
				frag_shader,
				r#"#version 460 core
				out vec4 color;
				void main() {
					color = vec4(0.7, 0.8, 0.9, 1.0);
				}"#,
			);
			gl.compile_shader(frag_shader);
			if !gl.get_shader_compile_status(frag_shader) {
				panic!("{}", gl.get_shader_info_log(frag_shader));
			}

			let program = gl.create_program().unwrap();
			gl.attach_shader(program, vert_shader);
			gl.attach_shader(program, frag_shader);
			gl.link_program(program);
			if !gl.get_program_link_status(program) {
				panic!("{}", gl.get_program_info_log(program));
			}
			gl.delete_shader(frag_shader);
			gl.delete_shader(vert_shader);

			let vbo = gl.create_buffer().unwrap();
			gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
			gl.buffer_data_u8_slice(ARRAY_BUFFER, as_bytes(&VERTICES), DYNAMIC_DRAW);

			let vao = gl.create_vertex_array().unwrap();
			gl.bind_vertex_array(Some(vao));
			gl.enable_vertex_attrib_array(0);
			gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, size_of::<Vertex>() as _, 0);

			Box::new(Self { vao, _vbo: vbo, program })
		}
	}
}

impl Plugin for BasicRenderer {
	fn render(&self, gl: &GlContext) {
		unsafe {
			gl.use_program(Some(self.program));
			gl.bind_vertex_array(Some(self.vao));
			gl.draw_arrays(TRIANGLES, 0, 3);
		}
	}
}
