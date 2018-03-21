#version 400 core

in vec2 fs_tex_pos;
flat in uint fs_blk_type;

uniform sampler2DArray texture_atlas;

out vec4 color;

void main() {
  if (fs_blk_type == 0)
  {
    discard;
  }

  color = texture(texture_atlas, vec3(fs_tex_pos, float(fs_blk_type - 1)));
}
