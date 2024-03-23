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
  pub color: FillColor,
  pub image: Option<ImageHandle>,
  pub top_left: FramePoint2d,
  pub bottom_right: FramePoint2d,
  pub corner_radius: Corners<f32>,
}

impl<T: Into<FillColor>> From<T> for RectLayer {
  fn from(color: T) -> Self {
    Self::from_color(color)
  }
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
      top_left: FramePoint2d::TOP_LEFT,
      bottom_right: FramePoint2d::BOTTOM_RIGHT,
      corner_radius: Corners::all(0.),
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
