#version 460 core
layout(location = 0) in vec3 in_position;
layout(location = 1) in vec2 in_tex_coord;
uniform mat4 mvp;
out vec2 tex_coord;
void main() {
	tex_coord = in_tex_coord;
	gl_Position = mvp * vec4(in_position, 1.0);
}