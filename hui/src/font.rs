
use alloc::vec::Vec;
use hui_painter::text::{FontHandle, DEFAULT_FONT};

pub struct FontStack {
  fonts: Vec<FontHandle>,
}

impl FontStack {
  pub fn new() -> Self {
    Self {
      #[cfg(not(feature = "default-font"))]
      fonts: Vec::new(),
      #[cfg(feature = "default-font")]
      fonts: vec![DEFAULT_FONT],
    }
  }

  pub fn push(&mut self, font: FontHandle) {
    self.fonts.push(font);
  }

  pub fn pop(&mut self) {
    assert!(self.fonts.pop().is_some())
  }

  pub fn current(&self) -> Option<FontHandle> {
    self.fonts.last().copied()
  }

  // pub fn current_or_default(&self) -> FontHandle {
  //   self.current().unwrap_or_default()
  // }
}

impl Default for FontStack {
  fn default() -> Self {
    Self::new()
  }
}
