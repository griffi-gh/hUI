//! element API and built-in elements like `Container`, `Button`, `Text`, etc.

use crate::{
  draw::{atlas::ImageCtx, UiDrawCommandList},
  input::InputCtx,
  layout::{LayoutInfo, Size2d},
  measure::Response,
  rect::Rect,
  signal::SignalStore,
  state::StateRepo,
  text::{FontHandle, TextMeasure},
  UiInstance,
};

mod builtin;
pub use builtin::*;

/// Context for the `Element::measure` function
pub struct MeasureContext<'a> {
  pub layout: &'a LayoutInfo,
  pub state: &'a StateRepo,
  pub text_measure: TextMeasure<'a>,
  pub current_font: FontHandle,
  pub images: ImageCtx<'a>,
  //XXX: should measure have a reference to input?
  //pub input: InputCtx<'a>,
}

/// Context for the `Element::process` function
pub struct ProcessContext<'a> {
  pub measure: &'a Response,
  pub layout: &'a LayoutInfo,
  pub draw: &'a mut UiDrawCommandList,
  pub state: &'a mut StateRepo,
  pub text_measure: TextMeasure<'a>,
  pub current_font: FontHandle,
  pub images: ImageCtx<'a>,
  pub input: InputCtx<'a>,
  pub signal: &'a mut SignalStore,
}

pub trait UiElement {
  /// Get the name of the element (in lower case)
  ///
  /// For example, "button" or "progress_bar"
  fn name(&self) -> &'static str;

  /// Get the requested UiElement size
  ///
  /// You should implement this function whenever possible, otherwise some features may not work at all, such as the `Remaining` size
  fn size(&self) -> Option<Size2d> { None }

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
  fn add_root(self, ui: &mut UiInstance, max_size: impl Into<Rect>);
}

impl<T: UiElement + 'static> UiElementExt for T {
  fn add_child(self, ui: &mut ElementList) {
    ui.add(self)
  }

  fn add_root(self, ui: &mut UiInstance, rect: impl Into<Rect>) {
    ui.add(self, rect);
  }
}
