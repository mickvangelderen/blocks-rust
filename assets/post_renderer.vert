#version 400 core

struct Frustrum {
  float x0;
  float x1;
  float y0;
  float y1;
  float z0;
  float z1;
};

uniform Frustrum frustrum;

in vec2 vs_ver_pos;
in vec2 vs_tex_pos;

out vec2 fs_tex_pos;
out vec2 fs_ray;

float linmap(float x, float x0, float x1, float y0, float y1) {
  return ((x - x0)*y1 + (x1 - x)*y0)/(x1 - x0);
}

void main() {
  gl_Position = vec4(vs_ver_pos, 0.0, 1.0);
  fs_tex_pos = vs_tex_pos;
  fs_ray = vec2(
    linmap(vs_tex_pos.x, 0.0, 1.0, frustrum.x0, frustrum.x1),
    linmap(vs_tex_pos.y, 1.0, 0.0, frustrum.y0, frustrum.y1)
  );
}
