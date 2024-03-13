//! wrapper that allows adding click and hover events to any element

// not sure if this is a good idea...
// but having the ability to add a click event to any element would be nice, and this is a naive way to do it

use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  signal::UiSignal,
};
use std::cell::RefCell;

#[non_exhaustive]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum InteractableEvent {
  #[default]
  Click,
  Hover,
}

/// Wrapper that allows adding click and hover events to any element
pub struct Interactable<C: UiSignal + 'static> {
  /// The wrapped element that will be interactable
  pub element: Box<dyn UiElement>,

  /// Event to listen for
  pub event: InteractableEvent,

  /// Signal that will be called if the element was clicked in the current frame
  ///
  /// Will be consumed after the first time it's called
  pub signal: RefCell<Option<C>>,
}

impl<C: UiSignal + 'static> Interactable<C> {
  pub fn new(element: Box<dyn UiElement>, event: InteractableEvent, signal: C) -> Self {
    Self {
      element,
      event,
      signal: RefCell::new(Some(signal)),
    }
  }
}

impl<C: UiSignal + 'static> UiElement for Interactable<C> {
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
      InteractableEvent::Click => ctx.input.check_click(rect),
      InteractableEvent::Hover => ctx.input.check_hover(rect),
    };

    if event_happened {
      if let Some(sig) = self.signal.take() {
        ctx.signal.add(sig);
      }
    }

    self.element.process(ctx)
  }
}

/// Extension trait for [`UiElement`] that adds methods to wrap the element in an [`Interactable`]
pub trait ElementInteractableExt: UiElement {
  /// Wrap the element in an [`Interactable`] that will call the given signal when the specified event occurs
  fn into_interactable<C: UiSignal + 'static>(self, event: InteractableEvent, signal: C) -> Interactable<C>;

  /// Wrap the element in an [`Interactable`] that will call the given signal when clicked
  fn on_click<C: UiSignal + 'static>(self, signal: C) -> Interactable<C>;

  /// Wrap the element in an [`Interactable`] that will call the given signal while hovered
  fn on_hover<C: UiSignal + 'static>(self, signal: C) -> Interactable<C>;
}

impl<T: UiElement + 'static> ElementInteractableExt for T {
  fn into_interactable<C: UiSignal + 'static>(self, event: InteractableEvent, signal: C) -> Interactable<C> {
    Interactable::new(Box::new(self), event, signal)
  }

  fn on_click<C: UiSignal + 'static>(self, signal: C) -> Interactable<C> {
    self.into_interactable(InteractableEvent::Click, signal)
  }

  fn on_hover<C: UiSignal + 'static>(self, signal: C) -> Interactable<C> {
    self.into_interactable(InteractableEvent::Hover, signal)
  }
}
