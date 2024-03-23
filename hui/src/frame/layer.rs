use glam::Vec2;
use enum_dispatch::enum_dispatch;
use crate::{
  color,
  draw::{ImageHandle, RoundedCorners, UiDrawCommand, UiDrawCommandList},
  rect::{Corners, FillColor},
};
use super::point::FramePoint2d;

#[enum_dispatch]
pub(crate) trait FrameLayerImpl {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2);
}

#[derive(Clone, Copy)]
#[enum_dispatch(FrameLayerImpl)]
pub enum FrameLayer {
  Rect(RectFrame),
}

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

impl<T: Into<FillColor>> From<T> for RectFrame {
  fn from(color: T) -> Self {
    Self::from_color(color)
  }
}

impl RectFrame {
  pub fn from_color(color: impl Into<FillColor>) -> Self {
    Self {
      color: color.into(),
      ..Self::default()
    }
  }

  pub fn from_color_rounded(color: impl Into<FillColor>, corner_radius: impl Into<Corners<f32>>) -> Self {
    Self {
      color: color.into(),
      corner_radius: corner_radius.into(),
      ..Self::default()
    }
  }

  pub fn from_image(image: ImageHandle) -> Self {
    Self {
      color: color::WHITE.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  pub fn from_color_image(color: impl Into<FillColor>, image: ImageHandle) -> Self {
    Self {
      color: color.into(),
      image: Some(image),
      ..Self::default()
    }
  }

  pub fn from_color_image_rounded(color: impl Into<FillColor>, image: ImageHandle, corner_radius: impl Into<Corners<f32>>) -> Self {
    Self {
      color: color.into(),
      image: Some(image),
      corner_radius: corner_radius.into(),
      ..Self::default()
    }
  }

  /// Inset the rectangle by the given amount
  pub fn inset(self, inset: f32) -> Self {
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
      color: FillColor::default(),
      image: None,
      top_left: FramePoint2d::TOP_LEFT,
      bottom_right: FramePoint2d::BOTTOM_RIGHT,
      corner_radius: Corners::all(0.),
    }
  }
}

impl FrameLayerImpl for RectFrame {
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
