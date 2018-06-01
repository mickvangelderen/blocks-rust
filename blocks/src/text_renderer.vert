#version 400 core

uniform mat4 pos_from_wld_to_clp_space;

in vec3 vs_ver_pos;
in vec2 vs_tex_pos;
in uint vs_char;
in vec2 vs_char_offset;

out vec2 fs_tex_pos;

void main() {
  float scale = 18.0f;

  mat4 pos_from_obj_to_wld_space = mat4(
    scale, 0.0, 0.0, 0.0,
    0.0, scale, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    vs_char_offset.x, vs_char_offset.y, 0.0, 1.0
  );

  gl_Position = pos_from_wld_to_clp_space*pos_from_obj_to_wld_space*vec4(vs_ver_pos, 1.0);

  // Compute texture coordinates for this instance.
  uint row = vs_char/16;
  uint col = vs_char - 16*row;
  vec2 tex_offset = vec2(float(col)/16.0, float(15 - row)/16.0);
  fs_tex_pos = vs_tex_pos + tex_offset;
}
