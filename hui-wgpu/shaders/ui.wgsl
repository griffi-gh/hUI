struct VertexInput {
  @location(0) position: vec3<f32>,
  @location(1) uv: vec2<f32>,
  @location(2) color: vec4<f32>,
}

struct VertexOutput {
  @builtin(position) clip_position: vec4<f32>,
  @location(0) uv: vec2<f32>,
  @location(1) color: vec4<f32>,
};

@vertex
fn vs_main(
  in: VertexInput,
) -> VertexOutput {
  var out: VertexOutput;
  out.clip_position = vec4<f32>(in.position, 1.0);
  out.uv = in.uv;
  out.color = in.color;
  return out;
}

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;

@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
  //HACK: This is a hack to convert the color to sRGB
  let srgb_color = pow(in.color, vec4<f32>(2.2, 2.2, 2.2, 1.0));
  return textureSample(t_diffuse, s_diffuse, in.uv) * srgb_color;
}
