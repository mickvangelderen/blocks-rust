#version 400 core

struct Frustrum {
  float x0;
  float x1;
  float y0;
  float y1;
  float z0;
  float z1;
};

uniform sampler2D color_texture;
uniform sampler2D depth_stencil_texture;
uniform Frustrum frustrum;
uniform vec2 viewport;
uniform vec2 mouse_pos;
uniform int mode;

in vec2 fs_tex_pos;
in vec2 fs_ray;

out vec4 color;

float linmap(float x, float x0, float x1, float y0, float y1) {
  return ((x - x0)*y1 + (x1 - x)*y0)/(x1 - x0);
}

float sample_z_ndc(vec2 tex_pos) {
  return texture(depth_stencil_texture, tex_pos).r * 2.0 - 1.0;
}

float z_from_ndc_to_cam_space(float z_ndc) {
  return (2.0*frustrum.z1*frustrum.z0) /
    (z_ndc*(frustrum.z1 - frustrum.z0) - (frustrum.z1 + frustrum.z0));
}

void main() {
  // Compute mouse coordinates in camera space.
  vec2 mouse_tex_pos = vec2(
    linmap(mouse_pos.x, 0.0, viewport.x, 0, 1),
    linmap(mouse_pos.y, viewport.y, 0.0, 0, 1)
  );
  vec2 mouse_ray = vec2(
    linmap(mouse_pos.x, 0.0, viewport.x, frustrum.x0, frustrum.x1),
    linmap(mouse_pos.y, 0.0, viewport.y, frustrum.y0, frustrum.y1)
  );
  float mouse_z_ndc = sample_z_ndc(mouse_tex_pos);
  float mouse_z_cam = z_from_ndc_to_cam_space(mouse_z_ndc);
  vec3 mouse_pos_cam = vec3(mouse_ray/frustrum.z0*mouse_z_cam, mouse_z_cam);

  // Compute fragment coordinates in camera space.
  float frag_z_ndc = sample_z_ndc(fs_tex_pos);
  float frag_z_cam = z_from_ndc_to_cam_space(frag_z_ndc);
  vec3 frag_pos_cam = vec3(fs_ray/frustrum.z0*frag_z_cam, frag_z_cam);

  if (mode == 0) {
    color = texture(color_texture, fs_tex_pos);
  } else if (mode == 1) {
    float d = linmap(frag_z_cam, -frustrum.z0, -frustrum.z1, 1.0, 0.0);
    color = vec4(vec3(d), 1.0);
  } else {
    color = texture(color_texture, fs_tex_pos);
    if (frag_pos_cam.x > -0.5 && frag_pos_cam.x < 0.5 && -frustrum.z1 < frag_pos_cam.z) {
      color = vec4(1.0, 0.0, 0.0, 1.0);
    }
  }

  // Mouse to frag vector.
  vec3 v = frag_pos_cam - mouse_pos_cam;

  // Inner and outer radius.
  float ri = 0.8;
  float ro = 1.0;

  // Squared distances.
  float d2 = dot(v, v);
  float ri2 = ri*ri;
  float ro2 = ro*ro;

  if (d2 >= ri2 && d2 < ro2) {
    color = vec4(normalize(v), 1.0);
  }
}
