use core::{hash::Hasher, num::NonZeroU16};
use glam::{vec2, Vec2};
use hui_shared::{color, rect::{Corners, FillColor, Rect}};
use crate::{
  paint::{
    buffer::{PaintBuffer, Vertex},
    command::PaintCommand,
  },
  texture::TextureHandle,
  util::{hash_vec2, hash_vec4},
  PainterInstance,
};

/// Calculate the number of points based on the maximum corner radius
fn point_count(corners: Corners<f32>) -> NonZeroU16 {
  //Increase for higher quality
  const VTX_PER_CORER_RADIUS_PIXEL: f32 = 0.5;
  NonZeroU16::new(
    (corners.max_f32() * VTX_PER_CORER_RADIUS_PIXEL).round() as u16 + 2
  ).unwrap()
}

pub struct PaintRectangle {
  /// Color of the rectangle.
  pub color: FillColor,

  /// Size of the rectangle.
  ///
  /// (Only different from using transform if the rectangle has border radius.)
  pub size: Vec2,

  /// Texture to use for the rectangle.
  ///
  /// Invalid handles will be ignored.
  pub texture: Option<TextureHandle>,

  /// UV coords inside the texture
  pub texture_uv: Corners<Vec2>,

  /// Border width.
  pub border_radius: Corners<f32>,

  // TODO per-corner border radius point count override

  /// Border radius point count.
  ///
  /// - If not set, it will be calculated based on the maximum radius.
  /// - If set, it will be used for all corners.
  pub border_radius_points_override: Option<NonZeroU16>,
}

impl Default for PaintRectangle {
  fn default() -> Self {
    Self {
      color: color::WHITE.into(),
      size: Vec2::ONE,
      texture: None,
      texture_uv: Corners {
        top_left: vec2(0., 0.),
        top_right: vec2(1., 0.),
        bottom_left: vec2(0., 1.),
        bottom_right: vec2(1., 1.),
      },
      border_radius: Corners::all(0.0),
      border_radius_points_override: None,
    }
  }
}

impl PaintRectangle {
  pub fn from_color(color: impl Into<FillColor>) -> Self {
    Self {
      color: color.into(),
      ..Default::default()
    }
  }

  pub fn from_texture(texture: TextureHandle) -> Self {
    Self {
      texture: Some(texture),
      color: color::WHITE.into(),
      ..Default::default()
    }
  }

  pub fn from_texture_color(texture: TextureHandle, color: impl Into<FillColor>) -> Self {
    Self {
      texture: Some(texture),
      color: color.into(),
      ..Default::default()
    }
  }
}

impl PaintCommand for PaintRectangle {
  fn paint(&self, ctx: &mut PainterInstance, into: &mut PaintBuffer) {
    // Offset from (0, 0) to the actual origin
    // We calculate positions in the range of [0, size] for simplicity
    // And then subtract this offset to get the actual position of a rectangle centered at (0, 0)
    // let origin_offset = self.size / 2.;

    // If texture is set:
    // - Get texture UV
    // - Map local UVs to texture UV coords
    // Otherwise, if texture handle is not set or invalid, use the bottom left
    // corner of the texture which contains a white pixel.
    let uvs = self.texture
      .and_then(|handle| ctx.textures.get_uv(handle))
      .map(|global_uv| {
        let texture_uv = self.texture_uv;
        let texture_uv_is_default =
          texture_uv.top_left == vec2(0., 0.) &&
          texture_uv.top_right == vec2(1., 0.) &&
          texture_uv.bottom_left == vec2(0., 1.) &&
          texture_uv.bottom_right == vec2(1., 1.);

        if texture_uv_is_default {
          global_uv
        } else {
          let top = global_uv.top_left
            .lerp(global_uv.top_right, texture_uv.top_left.x);
          let bottom = global_uv.bottom_left
            .lerp(global_uv.bottom_right, texture_uv.top_left.x);
          let top_left = top
            .lerp(bottom, texture_uv.top_left.y);

          let top = global_uv.top_left
            .lerp(global_uv.top_right, texture_uv.top_right.x);
          let bottom = global_uv.bottom_left
            .lerp(global_uv.bottom_right, texture_uv.top_right.x);
          let top_right = top
            .lerp(bottom, texture_uv.top_right.y);

          let top = global_uv.top_left
            .lerp(global_uv.top_right, texture_uv.bottom_left.x);
          let bottom = global_uv.bottom_left
            .lerp(global_uv.bottom_right, texture_uv.bottom_left.x);
          let bottom_left = top
            .lerp(bottom, texture_uv.bottom_left.y);

          let top = global_uv.top_left
            .lerp(global_uv.top_right, texture_uv.bottom_right.x);
          let bottom = global_uv.bottom_left
            .lerp(global_uv.bottom_right, texture_uv.bottom_right.x);
          let bottom_right = top
            .lerp(bottom, texture_uv.bottom_right.y);

          Corners { top_left, top_right, bottom_left, bottom_right }
        }
      })
      .unwrap_or(Corners::all(Vec2::ZERO)); // For non-textured rectangles

    // Get corner colors
    let colors = self.color.corners();

    // Get the base index for the vertices
    let idx_base = into.vertices.len() as u32;

    if self.border_radius.max_f32() == 0. {
      // No border radius:
      // Draw a simple quad (2 tris)
      let indices = Corners {
        top_left: idx_base,
        top_right: idx_base + 1,
        bottom_left: idx_base + 2,
        bottom_right: idx_base + 3,
      };
      into.indices.extend([
        indices.top_left,  indices.bottom_left, indices.top_right,
        indices.top_right, indices.bottom_left, indices.bottom_right,
      ]);
      into.vertices.extend([
        Vertex {
          position: vec2(0., 0.) * self.size, // - origin_offset,
          uv: uvs.top_left,
          color: colors.top_left,
        },
        Vertex {
          position: vec2(1., 0.) * self.size, // - origin_offset,
          uv: uvs.top_right,
          color: colors.top_right,
        },
        Vertex {
          position: vec2(0., 1.) * self.size, // - origin_offset,
          uv: uvs.bottom_left,
          color: colors.bottom_left,
        },
        Vertex {
          position: vec2(1., 1.) * self.size, // - origin_offset,
          uv: uvs.bottom_right,
          color: colors.bottom_right,
        },
      ]);
    } else {
      // Yes border radius :3
      // Draw a rounded rectangle with the given border radius and point count

      let point_count = self.border_radius_points_override
        .unwrap_or(point_count(self.border_radius))
        .get();

      // Get vertex for a point in scaled pixel space
      let point_impl = |point: Vec2| {
        let point_uv = point / self.size;
        let color_at_point =
          colors.bottom_right * point_uv.x * point_uv.y +
          colors.top_right * point_uv.x * (1. - point_uv.y) +
          colors.bottom_left * (1. - point_uv.x) * point_uv.y +
          colors.top_left * (1. - point_uv.x) * (1. - point_uv.y);
        let uv_at_point =
          uvs.bottom_right * point_uv.x * point_uv.y +
          uvs.top_right * point_uv.x * (1. - point_uv.y) +
          uvs.bottom_left * (1. - point_uv.x) * point_uv.y +
          uvs.top_left * (1. - point_uv.x) * (1. - point_uv.y);
        Vertex {
          position: point, // - origin_offset,
          color: color_at_point,
          uv: uv_at_point,
        }
      };

      into.vertices.reserve(point_count as usize * 4);
      into.indices.reserve((point_count as usize - 1) * 12 * 4);

      for i in 0..point_count as u32 {
        let frac = i as f32 / (point_count - 1) as f32;
        let angle = frac * core::f32::consts::PI * 0.5;
        let x = angle.sin();
        let y = angle.cos();
        into.vertices.extend([
          point_impl(vec2(x, 1. - y) * self.border_radius.top_right + vec2(self.size.x - self.border_radius.top_right, 0.)),
          point_impl(vec2(x - 1., y) * self.border_radius.bottom_right + vec2(self.size.x, self.size.y - self.border_radius.bottom_right)),
          point_impl(vec2(1. - x, y) * self.border_radius.bottom_left + vec2(0., self.size.y - self.border_radius.bottom_left)),
          point_impl(vec2(1. - x, 1. - y) * self.border_radius.top_left),
        ]);
        if i > 0 {
          // mental illness:
          into.indices.extend([
            //Top-right corner
            idx_base,
            idx_base + 1 + (i - 1) * 4,
            idx_base + 1 + i * 4,
            //Bottom-right corner
            idx_base,
            idx_base + 1 + (i - 1) * 4 + 1,
            idx_base + 1 + i * 4 + 1,
            //Bottom-left corner
            idx_base,
            idx_base + 1 + (i - 1) * 4 + 2,
            idx_base + 1 + i * 4 + 2,
            //Top-left corner
            idx_base,
            idx_base + 1 + (i - 1) * 4 + 3,
            idx_base + 1 + i * 4 + 3,
          ]);
        }

        //Fill in the rest
        //mental illness 2:
        into.indices.extend([
          //Top
          idx_base,
          idx_base + 4,
          idx_base + 1,
          //Right?, i think
          idx_base,
          idx_base + 1 + (point_count as u32 - 1) * 4,
          idx_base + 1 + (point_count as u32 - 1) * 4 + 1,
          //Left???
          idx_base,
          idx_base + 1 + (point_count as u32 - 1) * 4 + 2,
          idx_base + 1 + (point_count as u32 - 1) * 4 + 3,
          //Bottom???
          idx_base,
          idx_base + 3,
          idx_base + 2,
        ]);
      }

      // unimplemented!("Border radius is not supported yet");
    }
  }

  fn bounds(&self, _: &PainterInstance) -> Rect {
    // Rect {
    //   position: -self.size / 2.,
    //   size: self.size / 2.,
    // }
    Rect {
      position: Vec2::ZERO,
      size: self.size,
    }
  }

  fn cache_hash(&self) -> u64 {
    let mut hasher = rustc_hash::FxHasher::default();
    hash_vec2(&mut hasher, self.size);
    for corner in self.color.corners() {
      hash_vec4(&mut hasher, corner);
    }
    hasher.finish()
  }
}
