use glam::Vec4;
use crate::rectangle::Corners;

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub enum GradientDirection {
//   ToRight = 0b00,
//   ToLeft = 0b01,
//   ToBottom = 0b10,
//   ToTop = 0b11,
// }

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub enum Background {
  #[default]
  Transparent,
  Solid(Vec4),
  Gradient(Corners<Vec4>),
}

impl From<Vec4> for Background {
  fn from(color: Vec4) -> Self {
    Self::Solid(color)
  }
}

impl From<Option<Vec4>> for Background {
  fn from(color: Option<Vec4>) -> Self {
    match color {
      Some(color) => Self::Solid(color),
      None => Self::Transparent,
    }
  }
}

impl Background {
  /// Currently, never returns None.\
  /// `Option` has been added in preparation for future changes.\
  /// (`Background::Texture` etc)
  pub fn corners(&self) -> Option<Corners<Vec4>> {
    match *self {
      Self::Transparent => Some(Corners::all(Vec4::ZERO)),
      Self::Solid(color) => Some(Corners::all(color)),
      Self::Gradient(corners) => Some(corners),
    }
  }

  /// Returns `true` if the background is `Transparent` or all corners have an alpha value of `0`.
  pub fn is_transparent(&self) -> bool {
    match *self {
      Self::Transparent => true,
      Self::Solid(color) => color.w == 0.,
      Self::Gradient(corners) => {
        let max_alpha =
          corners.top_left.w
            .max(corners.top_right.w)
            .max(corners.bottom_left.w)
            .max(corners.bottom_right.w);
          max_alpha == 0.
      },
    }
  }
}

// impl From<(GradientDirection, Vec4, Vec4)> for Background {
//   fn from(gradient: (GradientDirection, Vec4, Vec4)) -> Self {
//     Self::Gradient(gradient.0, gradient.1, gradient.2)
//   }
// }
