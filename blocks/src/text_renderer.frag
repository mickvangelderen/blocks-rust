#version 400 core

uniform sampler2D font_texture;

in vec2 fs_tex_pos;

out vec4 color;

void fancy() {
  color = texture(font_texture, fs_tex_pos);
  float a_drop = texture(font_texture, fs_tex_pos + vec2(-1.0/160.0, 1.0/160.0)).a;

  if (color.a < 0.3) {
    if (a_drop > 0.5) {
      color = vec4(vec3(0.0), 1.0);
    } else {
      discard;
    };
  } else if (color.a < 0.4) {
    color = vec4(1.0);
  } else {
    color = vec4(color.rgb, 1.0);
  }
}

void bw() {
  color = texture(font_texture, fs_tex_pos);

  if (color.a < 0.2) {
    discard;
  } else if (color.a < 0.5) {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  } else {
    color = vec4(1.0, 1.0, 1.0, 1.0);
  }
}

void main() {
  fancy();
}
