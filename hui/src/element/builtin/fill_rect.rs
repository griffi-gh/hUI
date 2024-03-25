//! Simple filled rectangle with the specified size, background and corner radius

use derive_setters::Setters;
use glam::vec2;
use crate::{
  draw::{RoundedCorners, UiDrawCommand},
  element::{MeasureContext, ProcessContext, UiElement},
  frame::{Frame, FrameRect},
  layout::{compute_size, Size, Size2d},
  measure::Response,
  size
};

/// Simple filled rectangle with the specified size, background, and corner radius
#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct FillRect {
  /// Size of the rectangle
  #[setters(into)]
  pub size: Size2d,

  /// Frame
  #[setters(skip)]
  pub frame: Box<dyn Frame>,
}

impl FillRect {
  pub fn with_frame(mut self, frame: impl Frame + 'static) -> Self {
    self.frame = Box::new(frame);
    self
  }
}

impl Default for FillRect {
  fn default() -> Self {
    Self {
      size: size!(10, 10),
      frame: Box::new(FrameRect::color((0., 0., 0., 0.5))),
    }
  }
}

impl UiElement for FillRect {
  fn name(&self) -> &'static str {
    "fill_rect"
  }

  fn size(&self) -> Option<Size2d> {
    Some(self.size)
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: compute_size(ctx.layout, self.size, ctx.layout.max_size),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    // if !self.background.is_transparent() {
    //   ctx.draw.add(UiDrawCommand::Rectangle {
    //     position: ctx.layout.position,
    //     size: ctx.measure.size,
    //     color: self.background.corners(),
    //     texture: None,
    //     rounded_corners: (self.corner_radius.max_f32() > 0.).then_some({
    //       RoundedCorners::from_radius(self.corner_radius)
    //     }),
    //   });
    // }
    self.frame.draw(ctx.draw, ctx.layout.position, ctx.measure.size);
  }
}
