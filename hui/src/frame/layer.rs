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
  fn draw(&self, draw: &mut UiDrawCommandList, parent_size: Vec2);
}

#[derive(Clone, Copy)]
#[enum_dispatch(FrameLayerImpl)]
pub enum FrameLayer {
  Rect(RectLayer),
}

#[derive(Clone, Copy)]
pub struct RectLayer {
  color: FillColor,
  image: Option<ImageHandle>,
  top_left: FramePoint2d,
  bottom_right: FramePoint2d,
  corner_radius: Corners<f32>,
}

impl RectLayer {
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
}

impl Default for RectLayer {
  fn default() -> Self {
    Self {
      color: FillColor::default(),
      image: None,
      top_left: FramePoint2d::default(),
      bottom_right: FramePoint2d::default(),
      corner_radius: Corners::default(),
    }
  }
}

impl FrameLayerImpl for RectLayer {
  fn draw(&self, draw: &mut UiDrawCommandList, parent_size: Vec2) {
    //TODO: handle bottom_right < top_left
    let top_left = self.top_left.resolve(parent_size);
    let bottom_right = self.bottom_right.resolve(parent_size);
    draw.add(UiDrawCommand::Rectangle {
      position: top_left,
      size: bottom_right - top_left,
      color: self.color.corners(),
      texture: self.image,
      rounded_corners: (self.corner_radius.max_f32() > 0.).then_some(
        RoundedCorners::from_radius(self.corner_radius)
      ),
    });
  }
}
