use glam::Vec2;
use crate::{
  color,
  draw::{ImageHandle, RoundedCorners, UiDrawCommand, UiDrawCommandList},
  rect::{Corners, FillColor},
};
use super::{Frame, point::FramePoint2d};

#[derive(Clone, Copy)]
pub struct FrameRect {
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

// impl<T: Into<FillColor>> From<T> for FrameRect {
//   fn from(color: T) -> Self {
//     Self::from_color(color)
//   }
// }

impl From<FillColor> for FrameRect {
  fn from(color: FillColor) -> Self {
    Self::color(color)
  }
}

impl From<ImageHandle> for FrameRect {
  fn from(image: ImageHandle) -> Self {
    Self::image(image)
  }
}

impl FrameRect {
  /// Create a new [`FrameRect`] with the given color
  pub fn color(color: impl Into<FillColor>) -> Self {
    Self {
      color: color.into(),
      ..Self::default()
    }
  }

  /// Create a new [`FrameRect`] with the given image
  pub fn image(image: ImageHandle) -> Self {
    Self {
      color: color::WHITE.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  /// Create a new [`FrameRect`] with the given color and image
  pub fn color_image(color: impl Into<FillColor>, image: ImageHandle) -> Self {
    Self {
      color: color.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  /// Set the corner radius of the [`FrameRect`]
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

impl Default for FrameRect {
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

impl Frame for FrameRect {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    //TODO: handle bottom_right < top_left
    let top_left = self.top_left.resolve(parent_size);
    let bottom_right = self.bottom_right.resolve(parent_size);
    draw.add(UiDrawCommand::Rectangle {
      position: position + top_left,
      size: bottom_right - top_left,
      color: self.color.corners(),
      texture: self.image,
      rounded_corners: (self.corner_radius.max_f32() > 0.).then_some(
        RoundedCorners::from_radius(self.corner_radius)
      ),
    });
  }
}
