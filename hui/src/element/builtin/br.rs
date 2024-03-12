use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Br;

impl UiElement for Br {
  fn name(&self) -> &'static str {
    "Br"
  }

  fn measure(&self, _: MeasureContext) -> Response {
    Response {
      should_wrap: true,
      ..Default::default()
    }
  }

  fn process(&self, _: ProcessContext) {}
}
