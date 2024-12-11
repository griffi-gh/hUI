use fontdue::layout::GlyphRasterConfig;
use crate::texture::{TextureAtlas, TextureHandle};

pub(crate) mod ftm;
pub(crate) mod font;

pub use font::FontHandle;

pub struct FontManager {
  fonts: font::FontHandleManager,
  ftm: ftm::FontTextureManager,
}

impl FontManager {
  pub fn new() -> Self {
    Self {
      fonts: font::FontHandleManager::new(),
      ftm: ftm::FontTextureManager::new(),
    }
  }

  /// Add a font to the manager.
  ///
  /// Panics:
  /// - If the font data is invalid.
  pub fn add(&mut self, data: &[u8]) -> FontHandle {
    let font = self.fonts.add_font(data);
    self.ftm.init_font(font);
    font
  }

  /// Remove and deallocate a font from the manager.
  ///
  /// Panics:
  /// - If the font handle is invalid.
  pub fn remove(&mut self, font: FontHandle, atlas: &mut TextureAtlas) {
    self.ftm.drop_font(font, atlas);
    self.fonts.remove_font(font);
  }

  /// Render a glyph and cache it in the texture atlas.
  ///
  /// Panics:
  /// - If the font handle is invalid or not initialized.
  /// - Fuck around and find out, this api is unstable
  pub(crate) fn render_glyph(
    &mut self,
    atlas: &mut TextureAtlas,
    font: FontHandle,
    config: GlyphRasterConfig
  ) -> TextureHandle {
    self.ftm.render_glyph(font, &self.fonts, config, atlas)
  }

  /// Internal API
  pub(crate) fn get_fontdue_font(
    &self,
    handle: FontHandle
  ) -> Option<&fontdue::Font> {
    self.fonts.get_font_repr(handle)
      .map(|x| &x.font)
  }
}

impl Default for FontManager {
  fn default() -> Self {
    Self::new()
  }
}
