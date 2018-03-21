#version 400 core

uniform mat4 pos_from_wld_to_clp_space;

in vec3 vs_ver_pos;
in vec2 vs_tex_pos;

out vec2 fs_tex_pos;
flat out uint fs_blk_pos[3];

void main() {
  fs_blk_pos[0] = (gl_InstanceID >> 0) & 0xF;
  fs_blk_pos[1] = (gl_InstanceID >> 4) & 0xF;
  fs_blk_pos[2] = (gl_InstanceID >> 8) & 0xF;

  vec3 block_pos = vec3(
    fs_blk_pos[0],
    fs_blk_pos[1],
    fs_blk_pos[2]
  );

  mat4 pos_from_obj_to_wld_space = mat4(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 1.0, 0.0,
    block_pos.x, block_pos.y, block_pos.z, 1.0
  );

  gl_Position = pos_from_wld_to_clp_space*pos_from_obj_to_wld_space*vec4(vs_ver_pos, 1.0);
  fs_tex_pos = vs_tex_pos;
}
