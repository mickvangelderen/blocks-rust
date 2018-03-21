#version 400 core

in vec2 fs_tex_pos;
flat in uint fs_blk_type;

uniform sampler2D tex_1;
uniform sampler2D tex_2;

out vec4 color;

void main() {
  switch (fs_blk_type) {
    case 1:
      color = texture(tex_1, fs_tex_pos);
      break;
    case 2:
      color = texture(tex_2, fs_tex_pos);
      break;
    default:
      discard;
  }
}
