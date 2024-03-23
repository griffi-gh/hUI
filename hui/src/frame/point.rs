use glam::{Vec2, vec2};
use derive_more::{Add, AddAssign, Sub, SubAssign};
use crate::layout::{Size, Size2d};

/// Point inside a frame
///
/// Can be absolute, relative, or a combination of both\
/// Final coordinate is calculated as `absolute + relative * parent_size`
#[derive(Clone, Copy, Debug, Default, Add, AddAssign, Sub, SubAssign)]
pub struct FramePoint {
  /// Absolute positioning
  pub absolute: f32,

  /// Relative positioning
  pub relative: f32,
}

impl From<f32> for FramePoint {
  fn from(value: f32) -> Self {
    Self::absolute(value)
  }
}

impl From<Size> for FramePoint {
  /// Convert a `Size` into a `FramePoint`
  ///
  /// This function behaves just as you would expect, but `Auto` is always treated as `BEGIN`
  fn from(size: Size) -> Self {
    match size {
      Size::Auto => Self::BEGIN,
      Size::Relative(value) => Self::relative(value),
      Size::Absolute(value) => Self::absolute(value),
    }
  }
}

impl FramePoint {
  /// Beginning of the frame axis
  pub const BEGIN: Self = Self {
    absolute: 0.0,
    relative: 0.0,
  };

  /// Center of the frame axis
  pub const CENTER: Self = Self {
    absolute: 0.5,
    relative: 0.0,
  };

  /// End of the frame axis
  pub const END: Self = Self {
    absolute: 1.0,
    relative: 0.0,
  };

  /// Create a new absolutely positioned `FramePoint`
  pub const fn absolute(value: f32) -> Self {
    Self {
      absolute: value,
      relative: 0.0,
    }
  }

  /// Create a new relatively positioned `FramePoint`
  pub const fn relative(value: f32) -> Self {
    Self {
      absolute: 0.0,
      relative: value,
    }
  }

  /// Create a new `FramePoint` with both absolute and relative positioning
  pub const fn relative_absolute(relative: f32, absolute: f32) -> Self {
    Self {
      absolute,
      relative,
    }
  }

  /// Resolve the `FramePoint` into an absolute coordinate
  pub(crate) fn resolve(&self, parent_size: f32) -> f32 {
    self.absolute + self.relative * parent_size
  }
}

/// A 2-dimensional [`FramePoint`]
#[derive(Clone, Copy, Debug, Default, Add, AddAssign, Sub, SubAssign)]
pub struct FramePoint2d {
  pub x: FramePoint,
  pub y: FramePoint,
}

impl From<(FramePoint, FramePoint)> for FramePoint2d {
  fn from((x, y): (FramePoint, FramePoint)) -> Self {
    Self { x, y }
  }
}

impl From<Size> for FramePoint2d {
  fn from(size: Size) -> Self {
    Self {
      x: size.into(),
      y: size.into(),
    }
  }
}

impl From<Size2d> for FramePoint2d {
  fn from(size: Size2d) -> Self {
    Self {
      x: size.width.into(),
      y: size.height.into(),
    }
  }
}

impl From<(f32, f32)> for FramePoint2d {
  fn from((x, y): (f32, f32)) -> Self {
    Self {
      x: FramePoint::absolute(x),
      y: FramePoint::absolute(y),
    }
  }
}

impl From<Vec2> for FramePoint2d {
  fn from(vec: Vec2) -> Self {
    Self {
      x: FramePoint::absolute(vec.x),
      y: FramePoint::absolute(vec.y),
    }
  }
}

impl FramePoint2d {
  /// Top left corner of the frame
  pub const TOP_LEFT: Self = Self {
    x: FramePoint::BEGIN,
    y: FramePoint::BEGIN,
  };

  /// Top center of the frame
  pub const TOP_CENTER: Self = Self {
    x: FramePoint::CENTER,
    y: FramePoint::BEGIN,
  };

  /// Top right corner of the frame
  pub const TOP_RIGHT: Self = Self {
    x: FramePoint::END,
    y: FramePoint::BEGIN,
  };

  /// Center left of the frame
  pub const CENTER_LEFT: Self = Self {
    x: FramePoint::BEGIN,
    y: FramePoint::CENTER,
  };

  /// Center of the frame
  pub const CENTER: Self = Self {
    x: FramePoint::CENTER,
    y: FramePoint::CENTER,
  };

  /// Center right of the frame
  pub const CENTER_RIGHT: Self = Self {
    x: FramePoint::END,
    y: FramePoint::CENTER,
  };

  /// Bottom left corner of the frame
  pub const BOTTOM_LEFT: Self = Self {
    x: FramePoint::BEGIN,
    y: FramePoint::END,
  };

  /// Bottom center of the frame
  pub const BOTTOM_CENTER: Self = Self {
    x: FramePoint::CENTER,
    y: FramePoint::END,
  };

  /// Bottom right corner of the frame
  pub const BOTTOM_RIGHT: Self = Self {
    x: FramePoint::END,
    y: FramePoint::END,
  };

  /// Resolve the `FramePoint2d` into an absolute coordinate within the frame's coordinate system\
  ///
  /// (point with absolute cordinates, relative to the frame's top-left corner)
  pub(crate) fn resolve(&self, parent_size: Vec2) -> Vec2 {
    let x = self.x.resolve(parent_size.x);
    let y = self.y.resolve(parent_size.y);
    vec2(x, y)
  }
}
