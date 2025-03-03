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
  /// A rect with both position and size set to zero.
  pub const ZERO: Self = Self {
    position: Vec2::ZERO,
    size: Vec2::ZERO,
  };

  /// A rect with size of 1x1 and position of zero.
  pub const UNIT: Self = Self {
    position: Vec2::ZERO,
    size: Vec2::ONE,
  };

  pub const fn new(position: Vec2, size: Vec2) -> Self {
    Self { position, size }
  }

  pub const fn from_position(position: Vec2) -> Self {
    Self {
      position,
      size: Vec2::ZERO,
    }
  }

  pub const fn from_size(size: Vec2) -> Self {
    Self {
      position: Vec2::ZERO,
      size,
    }
  }

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

impl From<Vec2> for Rect {
  /// Create a new `Rect` from a `Vec2`, where x and y are the width and height of the rect respectively.
  fn from(size: Vec2) -> Self {
    Self::from_size(size)
  }
}

impl From<(Vec2, Vec2)> for Rect {
  /// Create a new `Rect` from a tuple of two `Vec2`s, where the first `Vec2` is the position and the second `Vec2` is the size.
  fn from((position, size): (Vec2, Vec2)) -> Self {
    Self { position, size }
  }
}

impl From<(f32, f32, f32, f32)> for Rect {
  /// Create a new `Rect` from a tuple of 4 `f32`s, where the first two `f32`s are the x and y positions of the top-left corner and the last two `f32`s are the width and height of the rect respectively.
  fn from((x, y, width, height): (f32, f32, f32, f32)) -> Self {
    Self {
      position: Vec2::new(x, y),
      size: Vec2::new(width, height),
    }
  }
}

impl From<[f32; 4]> for Rect {
  /// Create a new `Rect` from an array of 4 `f32`s, where the first two `f32`s are the x and y positions of the top-left corner and the last two `f32`s are the width and height of the rect respectively.
  fn from([x, y, width, height]: [f32; 4]) -> Self {
    Self {
      position: Vec2::new(x, y),
      size: Vec2::new(width, height),
    }
  }
}

impl From<Rect> for (Vec2, Vec2) {
  /// Convert a `Rect` into a tuple of two `Vec2`s, where the first `Vec2` is the position and the second `Vec2` is the size.
  fn from(rect: Rect) -> Self {
    (rect.position, rect.size)
  }
}

impl From<Rect> for (f32, f32, f32, f32) {
  /// Convert a `Rect` into a tuple of 4 `f32`s, where the first two `f32`s are the x and y positions of the top-left corner and the last two `f32`s are the width and height of the rect respectively.
  fn from(rect: Rect) -> Self {
    (rect.position.x, rect.position.y, rect.size.x, rect.size.y)
  }
}

impl From<Rect> for [f32; 4] {
  /// Convert a `Rect` into an array of 4 `f32`s, where the first two `f32`s are the x and y positions of the top-left corner and the last two `f32`s are the width and height of the rect respectively.
  fn from(rect: Rect) -> Self {
    [rect.position.x, rect.position.y, rect.size.x, rect.size.y]
  }
}
