#![doc(html_logo_url = "https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/hui.svg")]
//!
//! Simple UI library for games and other interactive applications
//!
//! # Features
#![doc = document_features::document_features!()]

#![allow(unused_parens)]
#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

mod instance;
pub mod layout;
pub mod rectangle;
pub mod element;
pub mod event;
pub mod input;
pub mod draw;
pub mod measure;
pub mod state;
pub mod text;

pub use instance::UiInstance;

pub trait IfModified<T> {
  fn if_modified(&self) -> Option<&T>;
}

#[allow(deprecated)]
#[deprecated(since = "0.1.0-alpha.3", note = "will be removed in the next release")]
pub struct ElementList(Vec<Box<dyn element::UiElement>>);

#[allow(deprecated)]
#[deprecated(since = "0.1.0-alpha.3", note = "will be removed in the next release")]
impl ElementList {
  pub fn add(&mut self, element: impl element::UiElement + 'static) {
    self.0.push(Box::new(element));
  }
}

#[allow(deprecated)]
#[deprecated(since = "0.1.0-alpha.3", note = "will be removed in the next release")]
pub fn elements(f: impl FnOnce(&mut ElementList)) -> Vec<Box<dyn element::UiElement>> {
  let mut elements = ElementList(Vec::new());
  f(&mut elements);
  elements.0
}
