//! Adds spacing between elements in a layout

use glam::vec2;
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response,
  layout::UiDirection
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
  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: match ctx.layout.direction {
        UiDirection::Horizontal => vec2(self.0, 0.),
        UiDirection::Vertical => vec2(0., self.0),
      },
      hints: Default::default(),
      user_data: None
    }
  }

  fn process(&self, _ctx: ProcessContext) {}
}
