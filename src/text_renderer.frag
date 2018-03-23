#version 400 core

uniform sampler2D font_texture;

in vec2 fs_tex_pos;

out vec4 color;

void main() {
  color = texture(font_texture, fs_tex_pos);
  if (color.a < 0.01)
  {
    discard;
  }
}
