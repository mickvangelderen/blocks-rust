#version 400 core

in vec2 vs_ver_pos;
in vec2 vs_tex_pos;

out vec2 fs_tex_pos;

void main() {
  gl_Position = vec4(vs_ver_pos, 0.0, 1.0);
  fs_tex_pos = vs_tex_pos;
}
