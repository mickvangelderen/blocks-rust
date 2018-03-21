#version 400 core

uniform mat4 pos_from_wld_to_clp_space;

in vec3 vs_ver_pos;
in vec2 vs_tex_pos;

out vec2 fs_tex_pos;
flat out uvec3 fs_blk_pos;

void main() {
  fs_blk_pos = uvec3(
    (gl_InstanceID >> 0) & 0xF,
    (gl_InstanceID >> 4) & 0xF,
    (gl_InstanceID >> 8) & 0xF
  );

  vec3 blk_pos_f = vec3(fs_blk_pos);

  mat4 pos_from_obj_to_wld_space = mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    blk_pos_f.x, blk_pos_f.y, blk_pos_f.z, 1.0
  );

  gl_Position = pos_from_wld_to_clp_space*pos_from_obj_to_wld_space*vec4(vs_ver_pos, 1.0);
  fs_tex_pos = vs_tex_pos;
}
