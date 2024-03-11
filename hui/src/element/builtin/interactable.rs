//! wrapper that allows adding click and hover events to any element

// not sure if this is a good idea...
// but having the ability to add a click event to any element would be nice, and this is a naive way to do it

use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  signal::{DummySignal, UiSignal},
};
use std::cell::RefCell;

/// Wrapper that allows adding click and hover events to any element
pub struct Interactable<H: UiSignal + 'static = DummySignal, C: UiSignal + 'static = DummySignal> {
  /// The wrapped element that will be interactable
  pub element: Box<dyn UiElement>,

  /// Signal that will be called if the element is hovered in the current frame
  ///
  /// Will be consumed after the first time it's called
  pub hovered: RefCell<Option<H>>,

  /// Signal that will be called if the element was clicked in the current frame
  ///
  /// Will be consumed after the first time it's called
  pub clicked: RefCell<Option<C>>,
}

impl<H: UiSignal, C: UiSignal> Interactable<H, C> {
  pub fn new(element: Box<dyn UiElement>) -> Self {
    Self {
      element,
      hovered: RefCell::new(None),
      clicked: RefCell::new(None),
    }
  }

  pub fn on_hover(self, hover: H) -> Self {
    Self {
      hovered: RefCell::new(Some(hover)),
      ..self
    }
  }

  pub fn on_click(self, clicked: C) -> Self {
    Self {
      clicked: RefCell::new(Some(clicked)),
      ..self
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
    //TODO other events...
    if ctx.input.check_click(rect) {
      if let Some(sig) = self.clicked.take() {
        //ctx.signal.push(sig);
      }
    }

    self.element.process(ctx)
  }
}

pub trait ElementInteractableExt: UiElement {
  fn into_interactable(self) -> Interactable;
}

impl<T: UiElement + 'static> ElementInteractableExt for T {
  fn into_interactable(self) -> Interactable {
    Interactable::new(Box::new(self))
  }
}
