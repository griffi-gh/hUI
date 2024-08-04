use glam::Vec2;
use hui_shared::rect::{Corners, FillColor};

pub struct PaintRectParams {
  /// Color of the rectangle.
  pub color: FillColor,

  /// Border width.
  pub border_radius: Corners<f32>,

  /// Border color.
  pub border_radius_points_override: Option<f32>,
}
