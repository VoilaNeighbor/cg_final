use std::mem::size_of;

use cg_final::framework::app::{App, GlContext, Plugin};
use cg_final::utils::as_bytes;
use glow::{
	HasContext, NativeBuffer, NativeProgram, NativeVertexArray, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT, FRAGMENT_SHADER, STATIC_DRAW, TRIANGLES, UNSIGNED_BYTE, VERTEX_SHADER
};

#[repr(C, packed)]
struct Vertex {
	position: [f32; 2],
}

const VERTICES: [Vertex; 4] = [
	Vertex { position: [-0.5, 0.5] },
	Vertex { position: [0.5, -0.5] },
	Vertex { position: [0.5, 0.5] },
	Vertex { position: [-0.5, -0.5] },
];

const ELEMENTS: [u8; 6] = [0, 2, 3, 3, 2, 1];

pub struct RectangleDemoRenderer {
	vao: NativeVertexArray,
	program: NativeProgram,
	_vbo: NativeBuffer,
	_ebo: NativeBuffer,
}

impl Plugin for RectangleDemoRenderer {
	fn render(&self, gl: &GlContext) {
		unsafe {
			gl.use_program(Some(self.program));
			gl.bind_vertex_array(Some(self.vao));
			gl.draw_elements(TRIANGLES, 6, UNSIGNED_BYTE, 0);
		}
	}
}

fn main() {
	App::default()
		.with_plugin(|gl| unsafe {
			let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
			gl.shader_source(
				vert_shader,
				r#"#version 460 core
				layout(location = 0) in vec2 position;
				out vec2 pos;
				void main() {
					pos = position;
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
				in vec2 pos;
				out vec4 color;
				void main() {
					color = vec4((pos.x + 1.) / 2., (pos.y + 1.) / 2., 0.9, 1.0);
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
			gl.buffer_data_u8_slice(ARRAY_BUFFER, as_bytes(&VERTICES), STATIC_DRAW);

			let vao = gl.create_vertex_array().unwrap();
			gl.bind_vertex_array(Some(vao));
			gl.enable_vertex_attrib_array(0);
			gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, size_of::<Vertex>() as _, 0);

			// Bind after VAO so that it is bound to the VAO, and we don't need to
			// bind it when rendering.
			let ebo = gl.create_buffer().unwrap();
			gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
			gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, as_bytes(&ELEMENTS), STATIC_DRAW);

			Box::new(RectangleDemoRenderer { vao, _vbo: vbo, program, _ebo: ebo })
		})
		.run();
}
