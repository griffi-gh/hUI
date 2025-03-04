use glam::{Affine2, Vec2};
use hui_painter::{paint::command::{PaintList, PaintRectangle, PaintTransform}, texture::TextureHandle};
use crate::{
  color,
  rect::{Rect, Corners, FillColor},
};
use super::{Frame, point::FramePoint2d};

/// A rectangular frame
///
/// Can optionally be tinted, textured, and have rounded corners
#[derive(Clone, Copy)]
pub struct RectFrame {
  /// Background color of the frame\
  ///
  /// If the container has a background texture, it will be multiplied by this color
  pub color: FillColor,

  /// Background texture of the frame
  ///
  /// Can be used in conjunction with the background color\
  /// In this case, the texture will be shaded by the color
  ///
  /// Please note that if the background color is NOT set (or set to transparent), the texture will NOT be visible\
  /// This is because the texture is multiplied by the color, and if the color is transparent, the texture will be too\
  pub image: Option<TextureHandle>,

  /// Top left corner of the rectangle
  pub top_left: FramePoint2d,

  /// Bottom right corner of the rectangle
  pub bottom_right: FramePoint2d,

  /// Corner radius of the frame
  pub corner_radius: Corners<f32>,
}

// impl<T: Into<FillColor>> From<T> for RectFrame {
//   fn from(color: T) -> Self {
//     Self::from_color(color)
//   }
// }

impl From<FillColor> for RectFrame {
  fn from(color: FillColor) -> Self {
    Self::color(color)
  }
}

impl From<TextureHandle> for RectFrame {
  fn from(image: TextureHandle) -> Self {
    Self::image(image)
  }
}

impl RectFrame {
  /// Create a new [`RectFrame`] with the given color
  pub fn color(color: impl Into<FillColor>) -> Self {
    Self {
      color: color.into(),
      ..Self::default()
    }
  }

  /// Create a new [`RectFrame`] with the given image\
  ///
  /// Color will be set to [`WHITE`](crate::color::WHITE) to ensure the image is visible
  pub fn image(image: TextureHandle) -> Self {
    Self {
      color: color::WHITE.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  /// Create a new [`RectFrame`] with the given color and image
  pub fn color_image(color: impl Into<FillColor>, image: TextureHandle) -> Self {
    Self {
      color: color.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  /// Set the corner radius of the [`RectFrame`]
  pub fn with_corner_radius(self, radius: impl Into<Corners<f32>>) -> Self {
    Self {
      corner_radius: radius.into(),
      ..self
    }
  }

  //TODO: deprecate and replace

  /// Inset the rectangle by the given amount in pixels
  pub fn with_inset(self, inset: f32) -> Self {
    Self {
      top_left: self.top_left + Vec2::splat(inset).into(),
      bottom_right: self.bottom_right - Vec2::splat(inset).into(),
      ..self
    }
  }
}

impl Default for RectFrame {
  fn default() -> Self {
    Self {
      color: FillColor::transparent(),
      image: None,
      top_left: FramePoint2d::TOP_LEFT,
      bottom_right: FramePoint2d::BOTTOM_RIGHT,
      corner_radius: Corners::all(0.),
    }
  }
}

impl Frame for RectFrame {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    if self.color.is_transparent() {
      return
    }
    //TODO: handle bottom_right < top_left
    let top_left = self.top_left.resolve(rect.size);
    let bottom_right = self.bottom_right.resolve(rect.size);
    draw.add(PaintTransform{
      transform: Affine2::from_translation(rect.position + top_left),
      child: PaintRectangle {
        size: bottom_right - top_left,
        color: self.color,
        texture: self.image,
        ..Default::default()
      },
    });
  }

  fn covers_opaque(&self) -> bool {
    self.top_left.x.relative <= 0. &&
    self.top_left.x.absolute <= 0. &&
    self.top_left.y.relative <= 0. &&
    self.top_left.y.absolute <= 0. &&
    self.bottom_right.x.relative >= 1. &&
    self.bottom_right.x.absolute >= 0. &&
    self.bottom_right.y.relative >= 1. &&
    self.bottom_right.y.absolute >= 0. &&
    self.color.is_opaque() &&
    self.image.is_none() &&
    self.corner_radius.max_f32() == 0.
  }
}
