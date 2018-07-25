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
  if (mode == 0) {
    color = texture(color_texture, fs_tex_pos);
  } else if (mode == 1) {
    float z_ndc = sample_z_ndc(fs_tex_pos);
    float z_cam = z_from_ndc_to_cam_space(z_ndc);
    float d = linmap(z_cam, -frustrum.z0, -frustrum.z1, 1.0, 0.0);
    color = vec4(vec3(d), 1.0);
  } else {
    float z_ndc = sample_z_ndc(fs_tex_pos);
    float z_cam = z_from_ndc_to_cam_space(z_ndc);
    vec3 pos_cam = vec3(fs_ray*z_cam, z_cam);
    color = texture(color_texture, fs_tex_pos);
    if (pos_cam.x > -0.5 && pos_cam.x < 0.5 && pos_cam.z < frustrum.z1) {
      color = vec4(1.0, 0.0, 0.0, 1.0);
    }
  }
}
