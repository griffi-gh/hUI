use glam::{Vec2, vec2};
use derive_more::{Add, AddAssign, Sub, SubAssign};
use crate::{
  draw::ImageHandle,
  element::fill_rect::FillRect,
  layout::{Size, Size2d}
};

//TODO finish dis, slider component would be a great place to test it

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
  pub const BEGIN: Self = Self {
    absolute: 0.0,
    relative: 0.0,
  };

  pub const CENTER: Self = Self {
    absolute: 0.5,
    relative: 0.0,
  };

  pub const END: Self = Self {
    absolute: 1.0,
    relative: 0.0,
  };

  pub const fn absolute(value: f32) -> Self {
    Self {
      absolute: value,
      relative: 0.0,
    }
  }

  pub const fn relative(value: f32) -> Self {
    Self {
      absolute: 0.0,
      relative: value,
    }
  }

  pub const fn relative_absolute(relative: f32, absolute: f32) -> Self {
    Self {
      absolute,
      relative,
    }
  }

  fn resolve(&self, parent_size: f32) -> f32 {
    self.absolute + self.relative * parent_size
  }
}

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
  pub const TOP_LEFT: Self = Self {
    x: FramePoint::BEGIN,
    y: FramePoint::BEGIN,
  };
  pub const TOP_CENTER: Self = Self {
    x: FramePoint::CENTER,
    y: FramePoint::BEGIN,
  };
  pub const TOP_RIGHT: Self = Self {
    x: FramePoint::END,
    y: FramePoint::BEGIN,
  };
  pub const CENTER_LEFT: Self = Self {
    x: FramePoint::BEGIN,
    y: FramePoint::CENTER,
  };
  pub const CENTER: Self = Self {
    x: FramePoint::CENTER,
    y: FramePoint::CENTER,
  };
  pub const CENTER_RIGHT: Self = Self {
    x: FramePoint::END,
    y: FramePoint::CENTER,
  };
  pub const BOTTOM_LEFT: Self = Self {
    x: FramePoint::BEGIN,
    y: FramePoint::END,
  };
  pub const BOTTOM_CENTER: Self = Self {
    x: FramePoint::CENTER,
    y: FramePoint::END,
  };
  pub const BOTTOM_RIGHT: Self = Self {
    x: FramePoint::END,
    y: FramePoint::END,
  };

  pub fn resolve(&self, parent_size: Vec2) -> Vec2 {
    let x = self.x.resolve(parent_size.x);
    let y = self.y.resolve(parent_size.y);
    vec2(x, y)
  }
}

enum FrameLayer {
  Rect {
    color: FillRect,
    image: Option<ImageHandle>,
    top_left: FramePoint2d,
    bottom_right: FramePoint2d,
  }
}

pub struct Frame {
  layers: Vec<FrameLayer>
}

impl<T: Into<FillRect>> From<T> for Frame {
  fn from(value: T) -> Self {
    todo!()
  }
}
