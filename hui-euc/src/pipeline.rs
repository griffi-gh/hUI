
use euc::{buffer::Buffer2d, Interpolate, Pipeline, Target};
use glam::{Vec2, Vec4};
use hui_painter::paint::buffer::Vertex;

pub struct UiPipeline {
  atlas: Buffer2d<[u8; 4]>,
}

#[derive(Clone, Copy)]
pub struct VsOut {
  position: Vec2,
  color: Vec4,
  uv: Vec2,
}

impl Interpolate for VsOut {
  fn lerp2(a: Self, b: Self, x: f32, y: f32) -> Self {
    Self {
      position: a.position.mul_add(Vec2::splat(x), y * b.position),
      color: a.color.mul_add(Vec4::splat(x), y * b.color),
      uv: a.uv.mul_add(Vec2::splat(x), y * b.uv),
    }
  }

  fn lerp3(a: Self, b: Self, c: Self, x: f32, y: f32, z: f32) -> Self {
    Self {
      position: a.position.mul_add(Vec2::splat(x), b.position.mul_add(Vec2::splat(y), z * c.position)),
      color: a.color.mul_add(Vec4::splat(x), b.color.mul_add(Vec4::splat(y), z * c.color)),
      uv: a.uv.mul_add(Vec2::splat(x), b.uv.mul_add(Vec2::splat(y), z * c.uv)),
    }
  }
}

impl Default for UiPipeline {
  fn default() -> Self {
    Self {
      atlas: Buffer2d::new([0, 0], [0; 4]),
    }
  }
}

impl Pipeline for UiPipeline {
  type Vertex = Vertex;
  type VsOut = VsOut;
  type Pixel = [u8; 4];

  // Vertex shader
  fn vert(&self, vtx: &Self::Vertex) -> ([f32; 4], Self::VsOut) {
    ([vtx.position.x, vtx.position.y, 0.0, 0.0], VsOut {
      position: vtx.position,
      color: vtx.color,
      uv: vtx.uv,
    })
  }

  // Fragment shader
  fn frag(&self, vs: &Self::VsOut) -> Self::Pixel {
    let color = vs.color.to_array().map(|x| (x * 255.).round() as u8);
    color //TODO sampling
    // match vs.uv != Vec2::ZERO {
    //   true => color * self.atlas.get(),
    //   false => color,
    // }
  }
}

