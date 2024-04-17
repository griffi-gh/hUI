use glam::Vec2;
use crate::{
  color,
  draw::{ImageHandle, RoundedCorners, UiDrawCommand, UiDrawCommandList},
  rect::{Corners, FillColor},
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
  pub image: Option<ImageHandle>,

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

impl From<ImageHandle> for RectFrame {
  fn from(image: ImageHandle) -> Self {
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
  pub fn image(image: ImageHandle) -> Self {
    Self {
      color: color::WHITE.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  /// Create a new [`RectFrame`] with the given color and image
  pub fn color_image(color: impl Into<FillColor>, image: ImageHandle) -> Self {
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
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    //TODO: handle bottom_right < top_left
    let top_left = self.top_left.resolve(parent_size);
    let bottom_right = self.bottom_right.resolve(parent_size);
    draw.add(UiDrawCommand::Rectangle {
      position: position + top_left,
      size: bottom_right - top_left,
      color: self.color.corners(),
      texture: self.image,
      texture_uv: None,
      rounded_corners: (self.corner_radius.max_f32() > 0.).then_some(
        RoundedCorners::from_radius(self.corner_radius)
      ),
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
