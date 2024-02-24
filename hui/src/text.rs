use std::sync::Arc;

mod font;
mod ftm;

use font::FontManager;
pub use font::FontHandle;
#[cfg(feature="builtin_font")]
pub use font::BUILTIN_FONT;
pub(crate) use font::DEFAULT_FONT;
use fontdue::{Font, FontSettings};
use ftm::FontTextureManager;
pub use ftm::{FontTextureInfo, GlyphCacheEntry};

use crate::draw::atlas::TextureAtlasManager;

pub struct TextRenderer {
  fm: FontManager,
  ftm: FontTextureManager,
}

impl TextRenderer {
  pub fn new() -> Self {
    Self {
      fm: FontManager::new(),
      ftm: FontTextureManager::default(),
    }
  }

  pub fn add_font_from_bytes(&mut self, font: &[u8]) -> FontHandle {
    self.fm.add_font(Font::from_bytes(font, FontSettings::default()).unwrap())
  }

  pub fn glyph(&mut self, atlas: &mut TextureAtlasManager, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
    self.ftm.glyph(&self.fm, atlas, font_handle, character, size)
  }

  pub(crate) fn internal_font(&self, handle: FontHandle) -> &Font {
    self.fm.get(handle).unwrap()
  }
}

impl Default for TextRenderer {
  fn default() -> Self {
    Self::new()
  }
}

pub struct TextMeasureResponse {
  pub max_width: f32,
  pub height: f32,
}

#[derive(Clone, Copy)]
pub struct TextMeasure<'a>(&'a TextRenderer);

impl<'a> TextMeasure<'a> {
  pub fn measure(&self, font: FontHandle, size: u8, text: &str) -> TextMeasureResponse {
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

  pub fn measure(&self, font: FontHandle, size: u8, text: &str) -> TextMeasureResponse {
    TextMeasure(self).measure(font, size, text)
  }
}
