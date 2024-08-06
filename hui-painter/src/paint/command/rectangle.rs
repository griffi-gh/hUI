use glam::{Vec2, vec2};
use hui_shared::{color, rect::{Corners, FillColor}};
use crate::paint::{
  buffer::PaintBuffer,
  command::PaintCommand,
};

pub struct PaintRectangle {
  /// Color of the rectangle.
  pub color: FillColor,

  /// Texture to use for the rectangle.
  pub texture: Option<u32>,

  /// UV coords inside the texture
  pub texture_uv: Corners<Vec2>,

  /// Border width.
  pub border_radius: Corners<f32>,

  /// Border color.
  pub border_radius_points_override: Option<f32>,
}

impl Default for PaintRectangle {
  fn default() -> Self {
    Self {
      color: color::WHITE.into(),
      texture: None,
      texture_uv: Corners {
        top_left: vec2(0., 0.),
        top_right: vec2(1., 0.),
        bottom_left: vec2(0., 1.),
        bottom_right: vec2(1., 1.),
      },
      border_radius: Corners::all(0.0),
      border_radius_points_override: None,
    }
  }
}

impl PaintCommand for PaintRectangle {
  fn paint(&self, into: &mut PaintBuffer) {
    todo!()
  }
}
