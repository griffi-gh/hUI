//! wrapper that allows adding click and hover events to any element

// not sure if this is a good idea...
// but having the ability to add a click event to any element would be nice, and this is a naive way to do it

use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  signal::{trigger::SignalTrigger, Signal},
};

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum InteractableEvent {
  #[default]
  Click,
  Hover,
  Active,
}

/// Wrapper that allows adding click and hover events to any element
pub struct Interactable {
  /// The wrapped element that will be interactable
  pub element: Box<dyn UiElement>,

  /// Event to listen for
  pub event: InteractableEvent,

  /// Signal that will be called if the element was clicked in the current frame
  pub signal: SignalTrigger,
}

impl Interactable {
  pub fn new<S: Signal, F: Fn() -> S + 'static>(
    element: Box<dyn UiElement>,
    event: InteractableEvent,
    signal: F
  ) -> Self {
    Self {
      element,
      event,
      signal: SignalTrigger::new(signal),
    }
  }
}

impl UiElement for Interactable {
  fn name(&self) -> &'static str {
    "Interactable"
  }

  fn measure(&self, ctx: MeasureContext) -> crate::measure::Response {
    self.element.measure(ctx)
  }

  fn process(&self, ctx: ProcessContext) {
    let rect = ctx.measure.rect(ctx.layout.position);

    //XXX: should we do this AFTER normal process call of wrapped element?
    let event_happened = match self.event {
      //TODO: actually pass the response
      InteractableEvent::Click => ctx.input.check_click(rect).is_some(),
      InteractableEvent::Hover => ctx.input.check_hover(rect),
      InteractableEvent::Active => ctx.input.check_active(rect).is_some(),
    };

    if event_happened {
      self.signal.fire(ctx.signal);
    }

    self.element.process(ctx)
  }
}

/// Extension trait for [`UiElement`] that adds methods to wrap the element in an [`Interactable`]
pub trait ElementInteractableExt: UiElement {
  /// Wrap the element in an [`Interactable`] that will call the given signal when the specified event occurs
  fn into_interactable<S: Signal, F: Fn() -> S + 'static>(self, event: InteractableEvent, signal: F) -> Interactable;

  /// Wrap the element in an [`Interactable`] that will call the given signal when clicked
  fn on_click<S: Signal, F: Fn() -> S + 'static>(self, signal: F) -> Interactable;

  /// Wrap the element in an [`Interactable`] that will call the given signal continuously while hovered
  fn on_hover<S: Signal, F: Fn() -> S + 'static>(self, signal: F) -> Interactable;

  /// Wrap the element in an [`Interactable`] that will call the given signal continuously while active
  fn on_active<S: Signal, F: Fn() -> S + 'static>(self, signal: F) -> Interactable;
}

impl<T: UiElement + 'static> ElementInteractableExt for T {
  fn into_interactable<S: Signal, F: Fn() -> S + 'static>(self, event: InteractableEvent, signal: F) -> Interactable {
    Interactable::new(Box::new(self), event, signal)
  }

  fn on_click<S: Signal, F: Fn() -> S + 'static>(self, signal: F) -> Interactable {
    self.into_interactable(InteractableEvent::Click, signal)
  }

  fn on_hover<S: Signal, F: Fn() -> S + 'static>(self, signal: F) -> Interactable {
    self.into_interactable(InteractableEvent::Hover, signal)
  }

  fn on_active<S: Signal, F: Fn() -> S + 'static>(self, signal: F) -> Interactable {
    self.into_interactable(InteractableEvent::Active, signal)
  }
}
