//! Contains types which represent the sides and corners of a rectangular shape.

use glam::Vec2;

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

/// Represents 4 sides of a rectangular shape.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub struct Sides<T> {
  pub top: T,
  pub bottom: T,
  pub left: T,
  pub right: T,
}

impl<T: Clone> Sides<T> {
  #[inline]
  pub fn all(value: T) -> Self {
    Self {
      top: value.clone(),
      bottom: value.clone(),
      left: value.clone(),
      right: value,
    }
  }

  #[inline]
  pub fn horizontal_vertical(horizontal: T, vertical: T) -> Self {
    Self {
      top: vertical.clone(),
      bottom: vertical,
      left: horizontal.clone(),
      right: horizontal,
    }
  }
}

impl<T: Clone> From<T> for Sides<T> {
  fn from(value: T) -> Self {
    Self::all(value)
  }
}

impl<T: Clone> From<(T, T)> for Sides<T> {
  fn from((horizontal, vertical): (T, T)) -> Self {
    Self::horizontal_vertical(horizontal, vertical)
  }
}

impl<T> From<(T, T, T, T)> for Sides<T> {
  fn from((top, bottom, left, right): (T, T, T, T)) -> Self {
    Self { top, bottom, left, right }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Corners<T> {
  pub top_left: T,
  pub top_right: T,
  pub bottom_left: T,
  pub bottom_right: T,
}

impl<T: Clone> Corners<T> {
  #[inline]
  pub fn all(value: T) -> Self {
    Self {
      top_left: value.clone(),
      top_right: value.clone(),
      bottom_left: value.clone(),
      bottom_right: value,
    }
  }

  #[inline]
  pub fn top_bottom(top: T, bottom: T) -> Self {
    Self {
      top_left: top.clone(),
      top_right: top,
      bottom_left: bottom.clone(),
      bottom_right: bottom,
    }
  }

  #[inline]
  pub fn left_right(left: T, right: T) -> Self {
    Self {
      top_left: left.clone(),
      top_right: right.clone(),
      bottom_left: left,
      bottom_right: right,
    }
  }
}

impl <T: Ord + Clone> Corners<T> {
  pub fn max(&self) -> T {
    self.top_left.clone()
      .max(self.top_right.clone())
      .max(self.bottom_left.clone())
      .max(self.bottom_right.clone())
      .clone()
  }
}

/// Represents 4 corners of a rectangular shape.
impl Corners<f32> {
  pub fn max_f32(&self) -> f32 {
    self.top_left
      .max(self.top_right)
      .max(self.bottom_left)
      .max(self.bottom_right)
  }
}

impl Corners<f64> {
  pub fn max_f64(&self) -> f64 {
    self.top_left
      .max(self.top_right)
      .max(self.bottom_left)
      .max(self.bottom_right)
  }
}

impl<T: Clone> From<T> for Corners<T> {
  fn from(value: T) -> Self {
    Self::all(value)
  }
}

impl<T> From<(T, T, T, T)> for Corners<T> {
  fn from((top_left, top_right, bottom_left, bottom_right): (T, T, T, T)) -> Self {
    Self {
      top_left,
      top_right,
      bottom_left,
      bottom_right,
    }
  }
}
