use glam::Vec2;
use super::Corners;

/// Represents a rectangle/AABB with specified position and size
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rect {
  /// Position of the top-left corner of the rect.
  pub position: Vec2,
  /// Size of the rect, should not be negative.
  pub size: Vec2,
}

impl Rect {
  /// Check if the rect contains a point.
  pub fn contains_point(&self, point: Vec2) -> bool {
    point.cmpge(self.position).all() && point.cmple(self.position + self.size).all()
  }

  //TODO: return intersect rect
  /// Check if the rect intersects with another rect.
  pub fn intersects_rect(&self, other: Rect) -> bool {
    self.position.x < other.position.x + other.size.x
      && self.position.x + self.size.x > other.position.x
      && self.position.y < other.position.y + other.size.y
      && self.position.y + self.size.y > other.position.y
  }

  /// Get width of the rectangle.
  ///
  /// To get both width and height, use the `size` property instead.
  pub fn width(&self) -> f32 {
    self.size.x
  }

  /// Get height of the rectangle.
  ///
  /// To get both width and height, use the `size` property instead.
  pub fn height(&self) -> f32 {
    self.size.y
  }

  /// Get position of the top-left corner of the rectangle on the x-axis.
  ///
  /// To get both x and y, use the `position` property instead.
  pub fn x(&self) -> f32 {
    self.position.x
  }

  /// Get position of the top-left corner of the rectangle on the y-axis.
  ///
  /// To get both x and y, use the `position` property instead.
  pub fn y(&self) -> f32 {
    self.position.y
  }

  /// Get positions of all 4 corners of the rectangle.
  pub fn corners(&self) -> Corners<Vec2> {
    Corners {
      top_left: self.position,
      top_right: self.position + Vec2::new(self.size.x, 0.0),
      bottom_left: self.position + Vec2::new(0.0, self.size.y),
      bottom_right: self.position + self.size,
    }
  }
}
