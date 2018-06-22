#version 400 core

uniform sampler2D color_texture;
uniform sampler2D depth_stencil_texture;
uniform float z_near;
uniform float z_far;

in vec2 fs_tex_pos;

out vec4 color;

// float linearize_depth(float value, float near, float far) {
//   return (2.0 * near * far) / (far + near - value * (far - near));
// }

// void main() {
//   float depth = linearize_depth(texture(depth_stencil_texture, fs_tex_pos).r, z_near, z_far);
//   color = vec4(vec3(depth), 1.0);
// }

void main() {
  color = vec4(texture(color_texture, fs_tex_pos).rgb, 1.0);
}
