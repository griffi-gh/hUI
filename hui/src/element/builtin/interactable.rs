//! wrapper that allows adding click and hover events to any element

// not sure if this is a good idea...
// but having the ability to add a click event to any element would be nice, and this is a naive way to do it

use crate::element::{UiElement, MeasureContext, ProcessContext};
use std::cell::RefCell;

/// Wrapper that allows adding click and hover events to any element
pub struct Interactable {
  /// The wrapped element that will be interactable
  pub element: Box<dyn UiElement>,
  /// Function that will be called if the element is hovered in the current frame
  ///
  /// Will be consumed after the first time it's called
  pub hovered: RefCell<Option<Box<dyn FnOnce()>>>,
  /// Function that will be called if the element was clicked in the current frame
  ///
  /// Will be consumed after the first time it's called
  pub clicked: RefCell<Option<Box<dyn FnOnce()>>>,
}

impl Interactable {
  pub fn new(element: Box<dyn UiElement>) -> Self {
    Self {
      element,
      hovered: RefCell::new(None),
      clicked: RefCell::new(None),
    }
  }

  pub fn on_click(self, clicked: impl FnOnce() + 'static) -> Self {
    Self {
      clicked: RefCell::new(Some(Box::new(clicked))),
      ..self
    }
  }

  pub fn on_hover(self, clicked: impl FnOnce() + 'static) -> Self {
    Self {
      clicked: RefCell::new(Some(Box::new(clicked))),
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
      //TODO better error message
      let clicked = self.clicked.borrow_mut().take().expect("you fucked up");
      clicked();
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
