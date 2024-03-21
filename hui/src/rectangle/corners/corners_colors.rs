use super::Corners;
use glam::{Vec3, Vec4, vec4};

/// Like Corners, but specialized for colors\
/// Opaque type, needs to be casted to `Corners<Vec4>` to be used
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CornersColors(Corners<Vec4>);

impl Default for CornersColors {
  fn default() -> Self {
    Self(Corners::all(vec4(0.0, 0.0, 0.0, 1.0)))
  }
}

impl From<Corners<Vec4>> for CornersColors {
  fn from(corners: Corners<Vec4>) -> Self {
    Self(corners)
  }
}

impl From<CornersColors> for Corners<Vec4> {
  fn from(corners: CornersColors) -> Self {
    corners.0
  }
}

impl From<Vec4> for CornersColors {
  fn from(value: Vec4) -> Self {
    Self(Corners::all(value))
  }
}

impl From<(f32, f32, f32, f32)> for CornersColors {
  fn from((r, g, b, a): (f32, f32, f32, f32)) -> Self {
    Self(Corners::all(vec4(r, g, b, a)))
  }
}

impl From<[f32; 4]> for CornersColors {
  fn from([r, g, b, a]: [f32; 4]) -> Self {
    Self(Corners::all(vec4(r, g, b, a)))
  }
}

impl From<Vec3> for CornersColors {
  fn from(value: Vec3) -> Self {
    Self(Corners::all(vec4(value.x, value.y, value.z, 1.0)))
  }
}

impl From<(f32, f32, f32)> for CornersColors {
  fn from((r, g, b): (f32, f32, f32)) -> Self {
    Self(Corners::all(vec4(r, g, b, 1.0)))
  }
}

impl From<[f32; 3]> for CornersColors {
  fn from([r, g, b]: [f32; 3]) -> Self {
    Self(Corners::all(vec4(r, g, b, 1.0)))
  }
}

impl From<(Vec4, Vec4, Vec4, Vec4)> for CornersColors {
  fn from((top_left, top_right, bottom_left, bottom_right): (Vec4, Vec4, Vec4, Vec4)) -> Self {
    Self(Corners { top_left, top_right, bottom_left, bottom_right })
  }
}

impl From<((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32))> for CornersColors {
  fn from(value: ((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32))) -> Self {
    Self(Corners {
      top_left: vec4(value.0.0, value.0.1, value.0.2, value.0.3),
      top_right: vec4(value.1.0, value.1.1, value.1.2, value.1.3),
      bottom_left: vec4(value.2.0, value.2.1, value.2.2, value.2.3),
      bottom_right: vec4(value.3.0, value.3.1, value.3.2, value.3.3),
    })
  }
}

impl From<[[f32; 4]; 4]> for CornersColors {
  fn from(value: [[f32; 4]; 4]) -> Self {
    Self(Corners {
      top_left: vec4(value[0][0], value[0][1], value[0][2], value[0][3]),
      top_right: vec4(value[1][0], value[1][1], value[1][2], value[1][3]),
      bottom_left: vec4(value[2][0], value[2][1], value[2][2], value[2][3]),
      bottom_right: vec4(value[3][0], value[3][1], value[3][2], value[3][3]),
    })
  }
}

impl From<(Vec3, Vec3, Vec3, Vec3)> for CornersColors {
  fn from((top_left, top_right, bottom_left, bottom_right): (Vec3, Vec3, Vec3, Vec3)) -> Self {
    Self(Corners {
      top_left: vec4(top_left.x, top_left.y, top_left.z, 1.0),
      top_right: vec4(top_right.x, top_right.y, top_right.z, 1.0),
      bottom_left: vec4(bottom_left.x, bottom_left.y, bottom_left.z, 1.0),
      bottom_right: vec4(bottom_right.x, bottom_right.y, bottom_right.z, 1.0),
    })
  }
}

impl From<((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32))> for CornersColors {
  fn from(value: ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32))) -> Self {
    Self(Corners {
      top_left: vec4(value.0.0, value.0.1, value.0.2, 1.0),
      top_right: vec4(value.1.0, value.1.1, value.1.2, 1.0),
      bottom_left: vec4(value.2.0, value.2.1, value.2.2, 1.0),
      bottom_right: vec4(value.3.0, value.3.1, value.3.2, 1.0),
    })
  }
}

impl From<[[f32; 3]; 4]> for CornersColors {
  fn from(value: [[f32; 3]; 4]) -> Self {
    Self(Corners {
      top_left: vec4(value[0][0], value[0][1], value[0][2], 1.0),
      top_right: vec4(value[1][0], value[1][1], value[1][2], 1.0),
      bottom_left: vec4(value[2][0], value[2][1], value[2][2], 1.0),
      bottom_right: vec4(value[3][0], value[3][1], value[3][2], 1.0),
    })
  }
}
