use super::FontHandle;

pub struct FontStack {
  fonts: Vec<FontHandle>,
}

impl FontStack {
  pub fn new() -> Self {
    Self {
      #[cfg(not(feature = "builtin_font"))]
      fonts: Vec::new(),
      #[cfg(feature = "builtin_font")]
      fonts: vec![super::BUILTIN_FONT],
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

  pub fn current_or_default(&self) -> FontHandle {
    self.current().unwrap_or_default()
  }
}
