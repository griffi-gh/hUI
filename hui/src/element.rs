//! element API and built-in elements like `Container`, `Button`, `Text`, etc.

use std::any::Any;
use crate::{
  draw::{atlas::ImageCtx, UiDrawCommandList},
  input::InputCtx,
  layout::LayoutInfo,
  measure::Response,
  signal::SignalStore,
  state::StateRepo,
  text::{FontHandle, TextMeasure},
  UiInstance,
};

mod builtin;
pub use builtin::*;

/// Context for the `Element::measure` function
pub struct MeasureContext<'a> {
  pub state: &'a StateRepo,
  pub layout: &'a LayoutInfo,
  pub text_measure: TextMeasure<'a>,
  pub current_font: FontHandle,
  pub images: ImageCtx<'a>,
  //XXX: should measure have a reference to input?
  //pub input: InputCtx<'a>,
}

/// Context for the `Element::process` function
pub struct ProcessContext<'a> {
  pub measure: &'a Response,
  pub state: &'a mut StateRepo,
  pub layout: &'a LayoutInfo,
  pub draw: &'a mut UiDrawCommandList,
  pub text_measure: TextMeasure<'a>,
  pub current_font: FontHandle,
  pub images: ImageCtx<'a>,
  pub input: InputCtx<'a>,
  pub signal: &'a mut SignalStore,
}

pub trait UiElement {
  /// Get the name of the element, for example "Button" or "ProgressBar"
  fn name(&self) -> &'static str;

  /// Get the unique id used for internal state management\
  /// This value must be unique for each instance of the element
  ///
  /// If the element is stateless, this function should return `None`
  fn state_id(&self) -> Option<u64> { None }

  /// Check if the element has state.\
  /// Should not be overridden
  fn is_stateful(&self) -> bool { self.state_id().is_some() }

  /// Check if the element has no state\
  /// Should not be overridden
  fn is_stateless(&self) -> bool { !self.is_stateful() }

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

/// A list of elements\
/// Use the [`add`](`ElementList::add`) method to add elements to the list
pub struct ElementList(pub Vec<Box<dyn UiElement>>);

impl ElementList {
  /// Add an element to the list
  pub fn add(&mut self, element: impl UiElement + 'static) {
    self.0.push(Box::new(element))
  }

  /// Create a new `ElementList` from a callback\
  /// The callback will be called with a reference to the newly list
  pub(crate) fn from_callback(cb: impl FnOnce(&mut ElementList)) -> Self {
    let mut list = ElementList(Vec::new());
    cb(&mut list);
    list
  }
}

/// Extension trait for [`UiElement`] that adds the [`add_child`] and [`add_root`] methods
pub trait UiElementExt: UiElement {
  /// Add element as a child/nested element.
  fn add_child(self, ui: &mut ElementList);

  /// Add element as a ui root.
  fn add_root(self, ui: &mut UiInstance, max_size: glam::Vec2);
}

impl<T: UiElement + 'static> UiElementExt for T {
  fn add_child(self, ui: &mut ElementList) {
    ui.add(self)
  }

  fn add_root(self, ui: &mut UiInstance, max_size: glam::Vec2) {
    ui.add(self, max_size);
  }
}
