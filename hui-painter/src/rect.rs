use glam::Vec2;
use hui_shared::rect::{Corners, FillColor};

pub struct PaintRectParams {
  /// Position of the top-left corner of the rectangle.
  pub position: Vec2,

  /// Position of the bottom-right corner of the rectangle.
  pub size: Vec2,

  /// Color of the rectangle.
  pub color: FillColor,

  /// Border width.
  pub border_radius: Corners<f32>,

  /// Border color.
  pub border_radius_points_override: Option<f32>,
}
