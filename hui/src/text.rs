//! text rendering, styling, measuring

use std::sync::Arc;
use fontdue::{Font, FontSettings};
use crate::draw::atlas::TextureAtlasManager;

mod font;
mod ftm;
mod stack;

/// Built-in font handle
#[cfg(feature="builtin_font")]
pub use font::BUILTIN_FONT;
pub use font::FontHandle;

use font::FontManager;
use ftm::FontTextureManager;
use ftm::GlyphCacheEntry;
use stack::FontStack;

pub(crate) struct TextRenderer {
  manager: FontManager,
  ftm: FontTextureManager,
  stack: FontStack,
}

impl TextRenderer {
  pub fn new() -> Self {
    Self {
      manager: FontManager::new(),
      ftm: FontTextureManager::default(),
      stack: FontStack::new(),
    }
  }

  pub fn add_font_from_bytes(&mut self, font: &[u8]) -> FontHandle {
    self.manager.add_font(Font::from_bytes(font, FontSettings::default()).unwrap())
  }

  pub fn glyph(&mut self, atlas: &mut TextureAtlasManager, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
    self.ftm.glyph(atlas, &self.manager, font_handle, character, size)
  }

  pub fn push_font(&mut self, font: FontHandle) {
    self.stack.push(font);
  }

  pub fn pop_font(&mut self) {
    self.stack.pop();
  }

  pub fn current_font(&self) -> FontHandle {
    self.stack.current_or_default()
  }

  pub(crate) fn internal_font(&self, handle: FontHandle) -> &Font {
    self.manager.get(handle).unwrap()
  }
}

impl Default for TextRenderer {
  fn default() -> Self {
    Self::new()
  }
}

/// Size of measured text
pub struct TextMeasureResponse {
  pub max_width: f32,
  pub height: f32,
}

/// Context for measuring text
#[derive(Clone, Copy)]
pub struct TextMeasure<'a>(&'a TextRenderer);

impl TextMeasure<'_> {
  /// Measure the given string of text with the given font and size
  pub fn measure(&self, font: FontHandle, size: u16, text: &str) -> TextMeasureResponse {
    use fontdue::layout::{Layout, CoordinateSystem, TextStyle};
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.append(
      &[self.0.internal_font(font)],
      &TextStyle::new(text, size as f32, 0)
    );
    TextMeasureResponse {
      max_width: layout.lines().map(|lines| {
        lines.iter().fold(0.0_f32, |acc, x| {
          let glyph = layout.glyphs().get(x.glyph_end).unwrap();
          acc.max(glyph.x + glyph.width as f32)
        })
      }).unwrap_or(0.),
      height: layout.height(),
    }
  }
}

impl TextRenderer {
  pub fn to_measure(&self) -> TextMeasure {
    TextMeasure(self)
  }

  pub fn measure(&self, font: FontHandle, size: u16, text: &str) -> TextMeasureResponse {
    TextMeasure(self).measure(font, size, text)
  }
}
