use std::num::NonZeroU16;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CornerRadius {
  pub top_left: f32,
  pub top_right: f32,
  pub bottom_left: f32,
  pub bottom_right: f32,
}

impl CornerRadius {
  pub const fn all(radius: f32) -> Self {
    Self {
      top_left: radius,
      top_right: radius,
      bottom_left: radius,
      bottom_right: radius,
    }
  }

  pub const fn none() -> Self {
    Self::all(0.0)
  }

  pub const fn top_bottom(top: f32, bottom: f32) -> Self {
    Self {
      top_left: top,
      top_right: top,
      bottom_left: bottom,
      bottom_right: bottom,
    }
  }

  pub const fn left_right(left: f32, right: f32) -> Self {
    Self {
      top_left: left,
      top_right: right,
      bottom_left: left,
      bottom_right: right,
    }
  }

  //XXX: should these be public? (don't see any reason to NOT expose them)
  pub fn max(&self) -> f32 {
    self.top_left
      .max(self.top_right)
      .max(self.bottom_left)
      .max(self.bottom_right)
  }

  pub fn point_count(&self) -> NonZeroU16 {
    //Increase for higher quality
    const VTX_PER_CORER_RADIUS_PIXEL: f32 = 0.5;
    NonZeroU16::new(
      (self.max() * VTX_PER_CORER_RADIUS_PIXEL).round() as u16 + 2
    ).unwrap()
  }
}

impl From<f32> for CornerRadius {
  fn from(radius: f32) -> Self {
    Self::all(radius)
  }
}

impl From<(f32, f32, f32, f32)> for CornerRadius {
  fn from((top_left, top_right, bottom_left, bottom_right): (f32, f32, f32, f32)) -> Self {
    Self {
      top_left,
      top_right,
      bottom_left,
      bottom_right,
    }
  }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedCorners {
  pub radius: CornerRadius,
  pub point_count: NonZeroU16,
}

impl RoundedCorners {
  pub fn from_radius(radius: CornerRadius) -> Self {
    Self {
      radius,
      point_count: radius.point_count(),
    }
  }
}

impl Default for RoundedCorners {
  fn default() -> Self {
    Self {
      radius: CornerRadius::default(),
      point_count: NonZeroU16::new(8).unwrap(),
    }
  }
}
