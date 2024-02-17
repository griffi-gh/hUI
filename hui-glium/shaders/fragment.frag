#version 300 es

precision highp float;
precision highp sampler2D;

out vec4 out_color;
in vec4 vtx_color;
in vec2 vtx_uv;

void main() {
  out_color = vtx_color;
}
