use super::Corners;
use glam::{Vec2, Vec3, Vec4, vec4};

/// Represents the fill color of a rectangle
///
/// Can be a single color or a simple gradient with different colors for each corner
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FillColor(Corners<Vec4>);

impl FillColor {
  pub const fn new(corners: Corners<Vec4>) -> Self {
    Self(corners)
  }

  /// Transparent background (alpha = 0)
  pub const TRANSPARENT: Self = Self::rgba(0., 0., 0., 0.);

  /// Transparent background (alpha = 0)
  pub const fn transparent() -> Self {
    Self::TRANSPARENT
  }

  /// Check if the fill color is completely transparent
  ///
  /// (i.e. all corners have an alpha value of 0.0)
  pub fn is_transparent(&self) -> bool {
    self.0.top_left.w == 0. &&
    self.0.top_right.w == 0. &&
    self.0.bottom_left.w == 0. &&
    self.0.bottom_right.w == 0.
  }

  /// Check if the fill color is completely opaque
  ///
  /// (i.e. all corners have an alpha value of 1.0)
  pub fn is_opaque(&self) -> bool {
    self.0.top_left.w == 1. &&
    self.0.top_right.w == 1. &&
    self.0.bottom_left.w == 1. &&
    self.0.bottom_right.w == 1.
  }

  /// Construct a solid color fill from values representing the red, green, blue and alpha channels
  pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
    Self(Corners {
      top_left: vec4(r, g, b, a),
      top_right: vec4(r, g, b, a),
      bottom_left: vec4(r, g, b, a),
      bottom_right: vec4(r, g, b, a),
    })
  }

  /// Construct a solid color fill from three values representing the red, green and blue channels
  pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
    Self(Corners {
      top_left: vec4(r, g, b, 1.0),
      top_right: vec4(r, g, b, 1.0),
      bottom_left: vec4(r, g, b, 1.0),
      bottom_right: vec4(r, g, b, 1.0),
    })
  }

  /// Construct a solid color fill from colors for each corner
  pub const fn from_corners(corners: Corners<Vec4>) -> Self {
    Self(corners)
  }

  /// Get a list of the colors for each corner
  pub const fn corners(&self) -> Corners<Vec4> {
    self.0
  }

  /// Interpolate color on position, assuming a linear gradient
  pub fn interpolate(&self, uv: Vec2) -> Vec4 {
    let c = self.corners();
    let top = c.top_left.lerp(c.top_right, uv.x);
    let bottom = c.bottom_left.lerp(c.bottom_right, uv.x);
    top.lerp(bottom, uv.y)
  }
}

impl Default for FillColor {
  fn default() -> Self {
    Self(Corners::all(vec4(0.0, 0.0, 0.0, 1.0)))
  }
}

impl From<Corners<Vec4>> for FillColor {
  fn from(corners: Corners<Vec4>) -> Self {
    Self(corners)
  }
}

impl From<FillColor> for Corners<Vec4> {
  fn from(corners: FillColor) -> Self {
    corners.0
  }
}

impl From<Vec4> for FillColor {
  fn from(value: Vec4) -> Self {
    Self(Corners::all(value))
  }
}

impl From<(f32, f32, f32, f32)> for FillColor {
  fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
    Self(Corners::all(vec4(r, g, b, a)))
  }
}

impl From<[f32; 4]> for FillColor {
  fn from([r, g, b, a]: [f32; 4]) -> Self {
    Self(Corners::all(vec4(r, g, b, a)))
  }
}

impl From<Vec3> for FillColor {
  fn from(value: Vec3) -> Self {
    Self(Corners::all(vec4(value.x, value.y, value.z, 1.0)))
  }
}

impl From<(f32, f32, f32)> for FillColor {
  fn from((r, g, b): (f32, f32, f32)) -> Self {
    Self(Corners::all(vec4(r, g, b, 1.0)))
  }
}

impl From<[f32; 3]> for FillColor {
  fn from([r, g, b]: [f32; 3]) -> Self {
    Self(Corners::all(vec4(r, g, b, 1.0)))
  }
}

impl From<(Vec4, Vec4, Vec4, Vec4)> for FillColor {
  fn from((top_left, top_right, bottom_left, bottom_right): (Vec4, Vec4, Vec4, Vec4)) -> Self {
    Self(Corners { top_left, top_right, bottom_left, bottom_right })
  }
}

impl From<((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32))> for FillColor {
  fn from(value: ((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32))) -> Self {
    Self(Corners {
      top_left: vec4(value.0.0, value.0.1, value.0.2, value.0.3),
      top_right: vec4(value.1.0, value.1.1, value.1.2, value.1.3),
      bottom_left: vec4(value.2.0, value.2.1, value.2.2, value.2.3),
      bottom_right: vec4(value.3.0, value.3.1, value.3.2, value.3.3),
    })
  }
}

impl From<[[f32; 4]; 4]> for FillColor {
  fn from(value: [[f32; 4]; 4]) -> Self {
    Self(Corners {
      top_left: vec4(value[0][0], value[0][1], value[0][2], value[0][3]),
      top_right: vec4(value[1][0], value[1][1], value[1][2], value[1][3]),
      bottom_left: vec4(value[2][0], value[2][1], value[2][2], value[2][3]),
      bottom_right: vec4(value[3][0], value[3][1], value[3][2], value[3][3]),
    })
  }
}

impl From<Corners<Vec3>> for FillColor {
  fn from(corners: Corners<Vec3>) -> Self {
    Self(Corners {
      top_left: corners.top_left.extend(1.),
      top_right: corners.top_right.extend(1.),
      bottom_left: corners.bottom_left.extend(1.),
      bottom_right: corners.bottom_right.extend(1.),
    })
  }
}

impl From<(Vec3, Vec3, Vec3, Vec3)> for FillColor {
  fn from((top_left, top_right, bottom_left, bottom_right): (Vec3, Vec3, Vec3, Vec3)) -> Self {
    Self(Corners {
      top_left: vec4(top_left.x, top_left.y, top_left.z, 1.0),
      top_right: vec4(top_right.x, top_right.y, top_right.z, 1.0),
      bottom_left: vec4(bottom_left.x, bottom_left.y, bottom_left.z, 1.0),
      bottom_right: vec4(bottom_right.x, bottom_right.y, bottom_right.z, 1.0),
    })
  }
}

impl From<((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32))> for FillColor {
  fn from(value: ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32))) -> Self {
    Self(Corners {
      top_left: vec4(value.0.0, value.0.1, value.0.2, 1.0),
      top_right: vec4(value.1.0, value.1.1, value.1.2, 1.0),
      bottom_left: vec4(value.2.0, value.2.1, value.2.2, 1.0),
      bottom_right: vec4(value.3.0, value.3.1, value.3.2, 1.0),
    })
  }
}

impl From<[[f32; 3]; 4]> for FillColor {
  fn from(value: [[f32; 3]; 4]) -> Self {
    Self(Corners {
      top_left: vec4(value[0][0], value[0][1], value[0][2], 1.0),
      top_right: vec4(value[1][0], value[1][1], value[1][2], 1.0),
      bottom_left: vec4(value[2][0], value[2][1], value[2][2], 1.0),
      bottom_right: vec4(value[3][0], value[3][1], value[3][2], 1.0),
    })
  }
}
