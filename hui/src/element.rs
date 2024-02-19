use std::any::Any;
use crate::{
  draw::UiDrawCommandList,
  measure::Response,
  state::StateRepo,
  text::TextMeasure,
  LayoutInfo
};

#[cfg(feature = "builtin_elements")]
mod builtin {
  pub mod rect;
  pub mod container;
  pub mod spacer;
  pub mod progress_bar;
  pub mod text;
}

#[cfg(feature = "builtin_elements")]
pub use builtin::*;

pub struct MeasureContext<'a> {
  pub state: &'a StateRepo,
  pub layout: &'a LayoutInfo,
  pub text_measure: TextMeasure<'a>,
}

pub struct ProcessContext<'a> {
  pub measure: &'a Response,
  pub state: &'a mut StateRepo,
  pub layout: &'a LayoutInfo,
  pub draw: &'a mut UiDrawCommandList,
  pub text_measure: TextMeasure<'a>,
}

pub trait UiElement {
  fn name(&self) -> &'static str { "UiElement" }
  fn state_id(&self) -> Option<u64> { None }
  fn is_stateful(&self) -> bool { self.state_id().is_some() }
  fn is_stateless(&self) -> bool { self.state_id().is_none() }
  fn init_state(&self) -> Option<Box<dyn Any>> { None }
  fn measure(&self, ctx: MeasureContext) -> Response;
  fn process(&self, ctx: ProcessContext);
}
