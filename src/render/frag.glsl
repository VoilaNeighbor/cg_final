#version 460 core
in vec2 tex_coord;
uniform sampler2D wall_tex;
uniform sampler2D face_tex;
out vec4 color;
void main() {
	color = mix(texture(wall_tex, tex_coord), texture(face_tex, tex_coord), 0.3);
}