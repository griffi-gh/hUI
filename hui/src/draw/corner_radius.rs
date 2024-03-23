use std::num::NonZeroU16;
use crate::rect::Corners;

//TODO uneven corners (separate width/height for each corner)

fn point_count(corners: Corners<f32>) -> NonZeroU16 {
  //Increase for higher quality
  const VTX_PER_CORER_RADIUS_PIXEL: f32 = 0.5;
  NonZeroU16::new(
    (corners.max_f32() * VTX_PER_CORER_RADIUS_PIXEL).round() as u16 + 2
  ).unwrap()
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoundedCorners {
  pub radius: Corners<f32>,
  pub point_count: NonZeroU16,
}

impl From<Corners<f32>> for RoundedCorners {
  fn from(radius: Corners<f32>) -> Self {
    Self::from_radius(radius)
  }
}

impl RoundedCorners {
  pub fn from_radius(radius: Corners<f32>) -> Self {
    Self {
      radius,
      point_count: point_count(radius),
    }
  }
}

impl Default for RoundedCorners {
  fn default() -> Self {
    Self {
      radius: Corners::default(),
      point_count: NonZeroU16::new(8).unwrap(),
    }
  }
}
