use std::f32::consts::PI;
use std::mem::size_of;
use std::time::Instant;

use cg_final::framework::app::{App, GlContext, Plugin};
use cg_final::utils::as_bytes;
use glow::{
	HasContext, NativeProgram, NativeTexture, ARRAY_BUFFER, ELEMENT_ARRAY_BUFFER, FLOAT, FRAGMENT_SHADER, NEAREST, RGBA, RGBA8, STATIC_DRAW, TEXTURE0, TEXTURE_2D, TEXTURE_MAG_FILTER, TEXTURE_MIN_FILTER, TRIANGLES, UNSIGNED_BYTE, VERTEX_SHADER
};
use nalgebra::{Matrix4, Unit, Vector3};

#[repr(C, packed)]
struct Vertex {
	position: [f32; 3],
	tex_coord: [f32; 2],
}

impl Vertex {
	/// # Safety
	/// VAO and VBO should be properly set up.
	unsafe fn format_attribs(gl: &GlContext) {
		gl.enable_vertex_attrib_array(0);
		gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, size_of::<Vertex>() as _, 0);
		gl.enable_vertex_attrib_array(1);
		gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, size_of::<Vertex>() as _, 12);
	}
}

const VERTICES: [Vertex; 24] = [
	// left
	Vertex { position: [-0.5, -0.5, -0.5], tex_coord: [0.0, 0.0] },
	Vertex { position: [-0.5, -0.5, 0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [-0.5, 0.5, 0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [-0.5, 0.5, -0.5], tex_coord: [1.0, 0.0] },
	// right
	Vertex { position: [0.5, -0.5, -0.5], tex_coord: [0.0, 0.0] },
	Vertex { position: [0.5, -0.5, 0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.5, 0.5, 0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [0.5, 0.5, -0.5], tex_coord: [1.0, 0.0] },
	// front
	Vertex { position: [-0.5, -0.5, 0.5], tex_coord: [0.0, 0.0] },
	Vertex { position: [-0.5, 0.5, 0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.5, 0.5, 0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [0.5, -0.5, 0.5], tex_coord: [1.0, 0.0] },
	// back
	Vertex { position: [-0.5, -0.5, -0.5], tex_coord: [0.0, 0.0] },
	Vertex { position: [-0.5, 0.5, -0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.5, 0.5, -0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [0.5, -0.5, -0.5], tex_coord: [1.0, 0.0] },
	// top
	Vertex { position: [-0.5, 0.5, -0.5], tex_coord: [0.0, 0.0] },
	Vertex { position: [-0.5, 0.5, 0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.5, 0.5, 0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [0.5, 0.5, -0.5], tex_coord: [1.0, 0.0] },
	// bottom
	Vertex { position: [-0.5, -0.5, -0.5], tex_coord: [0.0, 0.0] },
	Vertex { position: [-0.5, -0.5, 0.5], tex_coord: [0.0, 1.0] },
	Vertex { position: [0.5, -0.5, 0.5], tex_coord: [1.0, 1.0] },
	Vertex { position: [0.5, -0.5, -0.5], tex_coord: [1.0, 0.0] },
];

const ELEMENTS: [u8; 36] = [
	0, 1, 2, 2, 3, 0, // left
	4, 5, 6, 6, 7, 4, // right
	8, 9, 10, 10, 11, 8, // front
	12, 13, 14, 14, 15, 12, // back
	16, 17, 18, 18, 19, 16, // top
	20, 21, 22, 22, 23, 20, // bottom
];

struct Demo {
	program: NativeProgram,
	start: Instant,
}

impl Plugin for Demo {
	#[rustfmt::skip]
	fn render(&self, gl: &GlContext) {
		unsafe {
			let time = self.start.elapsed().as_secs_f32();

			let rotation_axis = Vector3::new(3.0, 5.0, 7.0);
			let rotation = Matrix4::from_axis_angle(&Unit::new_normalize(rotation_axis), time);
			let translation = Matrix4::new_translation(&Vector3::new(time.cos(), time.sin(), 0.0));

			let view = Matrix4::new(
				1.0, 0.0, 0.0, 0.0,
				0.0, 1.0, 0.0, 0.0,
				0.0, 0.0, 1.0, -3.0,
				0.0, 0.0, 0.0, 1.0,
			);

			// todo: Add a window plugin from which we can get window aspect.
			let projection = Matrix4::new_perspective(16.0 / 9.0, PI * 0.3, 0.1, 100.0);

			gl.uniform_matrix_4_f32_slice(
				gl.get_uniform_location(self.program, "mvp").as_ref(),
				false,
				(projection * view * translation * rotation).as_slice(),
			);

			gl.draw_elements(TRIANGLES, ELEMENTS.len() as i32, UNSIGNED_BYTE, 0);
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
				layout(location = 0) in vec3 in_position;
				layout(location = 1) in vec2 in_tex_coord;
				uniform mat4 mvp;
				out vec2 tex_coord;
				void main() {
					tex_coord = in_tex_coord;
					gl_Position = mvp * vec4(in_position, 1.0);
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

			gl.active_texture(TEXTURE0 + 1);
			autobind_texture(gl, include_bytes!("../../assets/textures/awesomeface.png"));
			gl.uniform_1_i32(gl.get_uniform_location(program, "face_tex").as_ref(), 1);

			Box::new(Demo { program, start: Instant::now() })
		})
		.run()
}
