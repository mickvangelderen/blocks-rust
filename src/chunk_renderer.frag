#version 400 core

in vec2 fs_tex_pos;
flat in uint fs_blk_pos[3];

uniform sampler2D tex;

out vec4 color;

void main() {
  color = texture(tex, fs_tex_pos);
  vec4 b = vec4(
    float(fs_blk_pos[0])/15.0,
    float(fs_blk_pos[1])/15.0,
    float(fs_blk_pos[2])/15.0,
    1.0
  );
  color = mix(color, b, 0.5);
}
