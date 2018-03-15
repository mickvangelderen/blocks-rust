#version 400 core

in vec2 fs_tex_pos;

uniform sampler2D tex;

out vec4 color;

void main() {
  color = texture(tex, fs_tex_pos);
}
