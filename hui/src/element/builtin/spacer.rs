//! Adds spacing between elements in a layout

use glam::vec2;
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response,
  layout::Direction
};

/// Adds spacing between elements in a layout\
/// (depending on the current layout direction)
pub struct Spacer(pub f32);

impl Default for Spacer {
  fn default() -> Self {
    Self(5.)
  }
}

impl UiElement for Spacer {
  fn name(&self) -> &'static str {
    "spacer"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: match ctx.layout.direction {
        Direction::Horizontal => vec2(self.0, 0.),
        Direction::Vertical => vec2(0., self.0),
      },
      ..Default::default()
    }
  }

  fn process(&self, _ctx: ProcessContext) {}
}
