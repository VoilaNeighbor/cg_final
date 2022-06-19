use std::mem::size_of;
use std::time::Instant;

use cg_final::framework::app::{App, GlContext, Plugin};
use cg_final::utils::as_bytes;
use glow::{
	HasContext, NativeBuffer, NativeProgram, NativeTexture, NativeVertexArray, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT, FRAGMENT_SHADER, NEAREST, RGBA, RGBA8, STATIC_DRAW, TEXTURE0, TEXTURE1, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TRIANGLES, UNSIGNED_BYTE, VERTEX_SHADER
};
use nalgebra::{Matrix4, Vector3};

#[repr(C, packed)]
struct Vertex {
	position: [f32; 2],
	tex_coord: [f32; 2],
}

impl Vertex {
	/// # Safety
	/// VAO and VBO should be properly set up.
	unsafe fn format_attribs(gl: &GlContext) {
		gl.enable_vertex_attrib_array(0);
		gl.vertex_attrib_pointer_f32(0, 2, FLOAT, false, size_of::<Vertex>() as _, 0);
		gl.enable_vertex_attrib_array(1);
		gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, size_of::<Vertex>() as _, 8);
	}
}

const VERTICES: [Vertex; 4] = [
	Vertex { position: [-0.5, 0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.5, -0.5], tex_coord: [1.0, 0.0] },
	Vertex { position: [0.5, 0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [-0.5, -0.5], tex_coord: [0.0, 0.0] },
];

const ELEMENTS: [u8; 6] = [0, 2, 3, 3, 2, 1];

struct DemoPlugin {
	program: NativeProgram,
	start: Instant,
}

impl Plugin for DemoPlugin {
	#[rustfmt::skip]
	fn render(&self, gl: &GlContext) {
		unsafe {
			let time = self.start.elapsed().as_secs_f32();
			let scale = Matrix4::new_nonuniform_scaling(&Vector3::new(
				time.cos() + 1.0,
				time.sin() + 1.0,
				1.0,
			));
			let rotation = Matrix4::new(
				time.cos(),  -time.sin(),  0.0, 0.0,
				time.sin(),  time.cos(),   0.0, 0.0,
				0.0,        0.0,           1.0, 0.0,
				0.0,        0.0,           0.0, 1.0,
			);
			let translation = Matrix4::new_translation(&Vector3::new(0.4, 0.4, 0.0));
			let transform = translation * rotation * scale;
			gl.uniform_matrix_4_f32_slice(
				gl.get_uniform_location(self.program, "transform").as_ref(),
				false,
				transform.data.as_slice(),
			);

			gl.draw_elements(TRIANGLES, 6, UNSIGNED_BYTE, 0);
		}
	}
}

/// Creates a new texture, bind to TEXTURE_2D, and return it.
///
/// # Safety
/// Need to activate before calling the texture unit in which you want it to sit.
///
/// # Example
/// ```no_run
/// gl.active_texture(TEXTURE1);
/// autobind_texture(gl, img_bytes);
/// gl.uniform_1_i32(gl.get_uniform_location(program, "tex").as_ref(), 1);
/// ```
unsafe fn autobind_texture(gl: &GlContext, img: &[u8]) -> NativeTexture {
	let img = image::io::Reader::new(std::io::Cursor::new(img))
		.with_guessed_format()
		.unwrap()
		.decode()
		.unwrap()
		.into_rgba8();
	let tex = gl.create_texture().unwrap();
	gl.bind_texture(TEXTURE_2D, Some(tex));
	gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MIN_FILTER, NEAREST as i32);
	gl.tex_parameter_i32(TEXTURE_2D, TEXTURE_MAG_FILTER, NEAREST as i32);
	// The 2 format parameters (RGBA vs RGBA8) are different. See:
	// https://www.khronos.org/registry/OpenGL-Refpages/gl4/html/glTexImage2D.xhtml
	gl.tex_image_2d(
		TEXTURE_2D,
		0,
		RGBA8 as i32,
		img.width() as i32,
		img.height() as i32,
		0,
		RGBA,
		UNSIGNED_BYTE,
		Some(&img),
	);
	tex
}

fn main() {
	App::default()
		.with_plugin(|gl| unsafe {
			let vao = gl.create_vertex_array().unwrap();
			gl.bind_vertex_array(Some(vao));

			let vbo = gl.create_buffer().unwrap();
			gl.bind_buffer(ARRAY_BUFFER, Some(vbo));
			gl.buffer_data_u8_slice(ARRAY_BUFFER, as_bytes(&VERTICES), STATIC_DRAW);

			Vertex::format_attribs(gl);

			// Bind after VAO so that it is bound to the VAO, and we don't need to bind it when rendering.
			let ebo = gl.create_buffer().unwrap();
			gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(ebo));
			gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, as_bytes(&ELEMENTS), STATIC_DRAW);

			let vert_shader = gl.create_shader(VERTEX_SHADER).unwrap();
			gl.shader_source(
				vert_shader,
				r#"#version 460 core
				layout(location = 0) in vec2 in_position;
				layout(location = 1) in vec2 in_tex_coord;
				uniform mat4 transform;
				out vec2 tex_coord;
				void main() {
					tex_coord = in_tex_coord;
					gl_Position = transform * vec4(in_position, 0.0, 1.0);
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
					color = mix(
						texture(wall_tex, tex_coord),
						texture(face_tex, tex_coord),
						0.3
					);
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

			gl.active_texture(TEXTURE0);
			autobind_texture(gl, include_bytes!("../../assets/textures/wall.jpg"));
			gl.uniform_1_i32(gl.get_uniform_location(program, "wall_tex").as_ref(), 0);

			gl.active_texture(TEXTURE1);
			autobind_texture(gl, include_bytes!("../../assets/textures/awesomeface.png"));
			gl.uniform_1_i32(gl.get_uniform_location(program, "face_tex").as_ref(), 1);

			Box::new(DemoPlugin { program, start: Instant::now() })
		})
		.run();
}
