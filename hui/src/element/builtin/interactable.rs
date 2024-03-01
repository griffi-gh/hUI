//TODO this thing?
//not sure if this is a good idea...
//but having the ability to add a click event to any element would be nice, and this is a naive way to do it

// use crate::element::{UiElement, MeasureContext, ProcessContext};

// pub struct Interactable<T: UiElement> {
//   pub element: T,
//   pub hovered: Option<Box<dyn FnOnce()>>,
//   pub clicked: Option<Box<dyn FnOnce()>>,
// }

// impl<T: UiElement> Interactable<T> {
//   pub fn new(element: T) -> Self {
//     Self {
//       element,
//       hovered: None,
//       clicked: None,
//     }
//   }

//   pub fn on_click(self, clicked: impl FnOnce() + 'static) -> Self {
//     Self {
//       clicked: Some(Box::new(clicked)),
//       ..self
//     }
//   }

//   pub fn on_hover(self, clicked: impl FnOnce() + 'static) -> Self {
//     Self {
//       clicked: Some(Box::new(clicked)),
//       ..self
//     }
//   }
// }

// impl<T: UiElement> UiElement for Interactable<T> {
//   fn measure(&self, ctx: MeasureContext) -> crate::measure::Response {
//     self.element.measure(ctx)
//   }

//   fn process(&self, ctx: ProcessContext) {
//     self.element.process(ctx)
//   }
// }

// pub trait IntoInteractable<T: UiElement>: UiElement {
//   fn into_interactable(self) -> Interactable<T>;
// }

// impl<T: UiElement> IntoInteractable<T> for T {
//   fn into_interactable(self) -> Interactable<Self> {
//     Interactable::new(self)
//   }
// }
