use std::any::Any;
use crate::{
  draw::UiDrawCommandList,
  measure::Response,
  state::StateRepo,
  text::TextMeasure,
  layout::LayoutInfo
};

mod builtin;
pub use builtin::*;

/// Context for the `Element::measure` function
pub struct MeasureContext<'a> {
  pub state: &'a StateRepo,
  pub layout: &'a LayoutInfo,
  pub text_measure: TextMeasure<'a>,
}

/// Context for the `Element::process` function
pub struct ProcessContext<'a> {
  pub measure: &'a Response,
  pub state: &'a mut StateRepo,
  pub layout: &'a LayoutInfo,
  pub draw: &'a mut UiDrawCommandList,
  pub text_measure: TextMeasure<'a>,
}

pub trait UiElement {
  /// Get the name of the element, for example "Button" or "ProgressBar"
  fn name(&self) -> &'static str { "UiElement" }

  /// Get the unique id used for internal state management\
  /// This value must be unique for each instance of the element
  ///
  /// If the element is stateless, this function should return `None`
  fn state_id(&self) -> Option<u64> { None }

  /// Check if the element has state
  fn is_stateful(&self) -> bool { self.state_id().is_some() }

  /// Check if the element has no state
  fn is_stateless(&self) -> bool { self.state_id().is_none() }

  /// Initialize the state of the element\
  /// This function should be called exactly once during the lifetime of the element,
  /// or if the state gets reset
  ///
  /// This function will not get called for stateless elements
  fn init_state(&self) -> Option<Box<dyn Any>> { None }

  /// Measure step, guaranteed to be called before the `process` step\
  /// May be called multiple times per single frame, so it should not contain any expensive calls\
  /// This function may not mutate any state.\
  ///
  /// This function should return the size of the element along with any hints or layout metadata
  fn measure(&self, ctx: MeasureContext) -> Response;

  /// Process step, guaranteed to be called after the `measure` step\
  /// You should  process the user inputs and render the element here.
  fn process(&self, ctx: ProcessContext);
}

pub struct ElementList(pub Vec<Box<dyn UiElement>>);

impl ElementList {
  pub fn add<'a>(&mut self, element: impl UiElement + 'static) {
    self.0.push(Box::new(element))
  }
}

impl<T: FnOnce(&mut ElementList)> From<T> for ElementList {
  fn from(cb: T) -> Self {
    let mut list = ElementList(Vec::new());
    cb(&mut list);
    list
  }
}

impl From<Vec<Box<dyn UiElement>>> for ElementList {
  fn from(value: Vec<Box<dyn UiElement>>) -> Self {
    Self(value)
  }
}

pub trait UiElementListExt {
  fn add_to(self, ui: &mut ElementList);
}

impl<T: UiElement + 'static> UiElementListExt for T {
  fn add_to(self, ui: &mut ElementList) {
    ui.add(self)
  }
}
