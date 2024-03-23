use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Break;

impl UiElement for Break {
  fn name(&self) -> &'static str {
    "Break"
  }

  fn measure(&self, _: MeasureContext) -> Response {
    Response {
      should_wrap: true,
      ..Default::default()
    }
  }

  fn process(&self, _: ProcessContext) {}
}
