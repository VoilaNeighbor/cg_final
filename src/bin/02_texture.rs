use std::io;
use std::mem::size_of;

use cg_final::framework::app::{App, GlContext, Plugin};
use cg_final::utils::as_bytes;
use glow::{
	HasContext, NativeBuffer, NativeProgram, NativeTexture, NativeVertexArray, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT, FRAGMENT_SHADER, NEAREST, RGBA, RGBA8, STATIC_DRAW, TEXTURE0, TEXTURE1, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TRIANGLES, UNSIGNED_BYTE, VERTEX_SHADER
};

#[repr(C, packed)]
struct Vertex {
	position: [f32; 2],
	tex_coord: [f32; 2],
}

const VERTICES: [Vertex; 4] = [
	Vertex { position: [-0.75, 0.75], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.75, -0.75], tex_coord: [1.0, 0.0] },
	Vertex { position: [0.75, 0.75], tex_coord: [1.0, 1.0] },
	Vertex { position: [-0.75, -0.75], tex_coord: [0.0, 0.0] },
];

const ELEMENTS: [u8; 6] = [0, 2, 3, 3, 2, 1];

pub struct TextureDemoRenderer {
	vao: NativeVertexArray,
	program: NativeProgram,
	_vbo: NativeBuffer,
	_ebo: NativeBuffer,

	wall_tex: NativeTexture,
	face_tex: NativeTexture,
}

impl Plugin for TextureDemoRenderer {
	fn render(&self, gl: &GlContext) {
		unsafe {
			gl.active_texture(TEXTURE0);
			gl.bind_texture(TEXTURE_2D, Some(self.wall_tex));
			gl.active_texture(TEXTURE1);
			gl.bind_texture(TEXTURE_2D, Some(self.face_tex));

			gl.use_program(Some(self.program));
			gl.bind_vertex_array(Some(self.vao));
			gl.draw_elements(TRIANGLES, 6, UNSIGNED_BYTE, 0);
		}
	}
}

fn main() {
	App::default()
		.with_plugin(|gl| unsafe {
			let vao = gl.create_vertex_array().unwrap();
			gl.bind_vertex_array(Some(vao));

			let vbo = gl.create_buffer().unwrap();
			gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
			gl.buffer_data_u8_slice(ARRAY_BUFFER, as_bytes(&VERTICES), STATIC_DRAW);

			// Vertex Attributes after both VAO and VBO are properly set up.
			gl.enable_vertex_attrib_array(0);
			gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, size_of::<Vertex>() as _, 0);
			gl.enable_vertex_attrib_array(1);
			gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, size_of::<Vertex>() as _, 8);

			// Bind after VAO so that it is bound to the VAO, and we don't need to bind it when rendering.
			let ebo = gl.create_buffer().unwrap();
			gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
			gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, as_bytes(&ELEMENTS), STATIC_DRAW);

			const WALL_ASSET: &[u8] = include_bytes!("../../assets/textures/wall.jpg");
			let wall_img = image::io::Reader::new(io::Cursor::new(WALL_ASSET))
				.with_guessed_format()
				.unwrap()
				.decode()
				.unwrap()
				.into_rgba8();
			let wall_tex = gl.create_texture().unwrap();
			gl.active_texture(TEXTURE0);
			gl.bind_texture(TEXTURE_2D, Some(wall_tex));
			gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
			gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
			// The 2 format parameters are different. See:
			// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml
			gl.tex_image_2d(
				TEXTURE_2D,
				0,
				RGBA8 as i32,
				wall_img.width() as i32,
				wall_img.height() as i32,
				0,
				RGBA,
				UNSIGNED_BYTE,
				Some(&wall_img),
			);

			const FACE_ASSET: &[u8] = include_bytes!("../../assets/textures/awesomeface.png");
			let face_img = image::io::Reader::new(io::Cursor::new(FACE_ASSET))
				.with_guessed_format()
				.unwrap()
				.decode()
				.unwrap()
				.flipv()
				.into_rgba8();
			let face_tex = gl.create_texture().unwrap();
			gl.active_texture(TEXTURE1);
			gl.bind_texture(TEXTURE_2D, Some(face_tex));
			gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
			gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
			gl.tex_image_2d(
				TEXTURE_2D,
				0,
				RGBA8 as i32,
				face_img.width() as i32,
				face_img.height() as i32,
				0,
				RGBA,
				UNSIGNED_BYTE,
				Some(&face_img),
			);

			let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
			gl.shader_source(
				vert_shader,
				r#"#version 460 core
				layout(location = 0) in vec2 in_position;
				layout(location = 1) in vec2 in_tex_coord;
				out vec2 tex_coord;
				void main() {
					tex_coord = in_tex_coord;
					gl_Position = vec4(in_position, 0.0, 1.0);
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
				in vec2 tex_coord;
				uniform sampler2D wall_tex;
				uniform sampler2D face_tex;
				out vec4 color;
				void main() {
					color = mix(texture(wall_tex, tex_coord), texture(face_tex, tex_coord), 0.2);
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

			gl.use_program(Some(program));
			gl.uniform_1_i32(gl.get_uniform_location(program, "wall_tex").as_ref(), 0);
			gl.uniform_1_i32(gl.get_uniform_location(program, "face_tex").as_ref(), 1);

			Box::new(TextureDemoRenderer { vao, program, _vbo: vbo, _ebo: ebo, wall_tex, face_tex })
		})
		.run();
}
