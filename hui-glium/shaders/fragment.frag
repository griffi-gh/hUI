#version 300 es

precision highp float;
precision highp sampler2D;

out vec4 out_color;
in vec4 vtx_color;
in vec2 vtx_uv;
uniform sampler2D tex;

void main() {
  //HACK: if vtx_uv is (0, 0) then the texture is not used
  if (vtx_uv.x == 0.0 && vtx_uv.y == 0.0) {
    out_color = vtx_color;
    return;
  }
  out_color = texture(tex, vtx_uv) * vtx_color;
}
