use glam::{vec4, Vec3, Vec4};
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

impl From<(f32, f32, f32, f32)> for Background {
  fn from(color: (f32, f32, f32, f32)) -> Self {
    Self::Solid(vec4(color.0, color.1, color.2, color.3))
  }
}

impl From<Corners<Vec4>> for Background {
  fn from(corners: Corners<Vec4>) -> Self {
    Self::Gradient(corners)
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

impl From<Vec4> for Background {
  fn from(color: Vec4) -> Self {
    Self::Solid(color)
  }
}

impl From<(f32, f32, f32)> for Background {
  fn from(color: (f32, f32, f32)) -> Self {
    Self::Solid(vec4(color.0, color.1, color.2, 1.))
  }
}

impl From<Corners<Vec3>> for Background {
  fn from(corners: Corners<Vec3>) -> Self {
    Self::Gradient(Corners {
      top_left: corners.top_left.extend(1.),
      top_right: corners.top_right.extend(1.),
      bottom_left: corners.bottom_left.extend(1.),
      bottom_right: corners.bottom_right.extend(1.),
    })
  }
}

impl From<Option<Vec3>> for Background {
  fn from(color: Option<Vec3>) -> Self {
    match color {
      Some(color) => Self::Solid(color.extend(1.)),
      None => Self::Transparent,
    }
  }
}

impl From<Vec3> for Background {
  fn from(color: Vec3) -> Self {
    Self::Solid(color.extend(1.))
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
