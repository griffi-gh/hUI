//! Simple element that displays the specified frame

use derive_setters::Setters;
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  frame::{Frame, RectFrame},
  layout::{compute_size, Size2d},
  measure::Response,
  size
};

/// Simple rectangle that displays the specified frame
#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct FrameView {
  /// Size of the rectangle
  #[setters(into)]
  pub size: Size2d,

  /// Frame
  #[setters(skip)]
  pub frame: Box<dyn Frame>,
}

impl FrameView {
  pub fn new(frame: impl Frame + 'static) -> Self {
    Self {
      size: size!(10, 10),
      frame: Box::new(frame),
    }
  }

  //setters:
  pub fn with_frame(mut self, frame: impl Frame + 'static) -> Self {
    self.frame = Box::new(frame);
    self
  }
}

impl Default for FrameView {
  fn default() -> Self {
    Self {
      size: size!(10, 10),
      frame: Box::new(RectFrame::color((0., 0., 0., 0.5))),
    }
  }
}

impl UiElement for FrameView {
  fn name(&self) -> &'static str {
    "frame_view"
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
    self.frame.draw(ctx.paint_target, (ctx.layout.position, ctx.measure.size).into());
  }
}
