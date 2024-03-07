use std::sync::Arc;
use fontdue::Metrics;
use hashbrown::HashMap;
use crate::draw::atlas::{TextureAtlasManager, ImageHandle};

use super::font::{FontHandle, FontManager};

#[derive(PartialEq, Eq, Hash)]
struct GlyphCacheKey {
  font_index: usize,
  character: char,
  size: u8,
}

pub struct GlyphCacheEntry {
  pub metrics: Metrics,
  pub texture: ImageHandle,
}

pub struct FontTextureManager {
  glyph_cache: HashMap<GlyphCacheKey, Arc<GlyphCacheEntry>>
}

impl FontTextureManager {
  pub fn new() -> Self {
    FontTextureManager {
      glyph_cache: HashMap::new(),
    }
  }

  /// Either looks up the glyph in the cache or renders it and adds it to the cache.
  pub fn glyph(
    &mut self,
    atlas: &mut TextureAtlasManager,
    font_manager: &FontManager,
    font_handle: FontHandle,
    character: char,
    size: u8
  ) -> Arc<GlyphCacheEntry> {
    let key = GlyphCacheKey {
      font_index: font_handle.0,
      character,
      size,
    };
    if let Some(entry) = self.glyph_cache.get(&key) {
      return Arc::clone(entry);
    }
    let font = font_manager.get(font_handle).unwrap();
    let (metrics, bitmap) = font.rasterize(character, size as f32);
    log::trace!("rasterized glyph: {}, {:?}, {:?}", character, metrics, bitmap);
    let texture = atlas.add_grayscale(metrics.width, &bitmap);
    let entry = Arc::new(GlyphCacheEntry {
      metrics,
      texture
    });
    self.glyph_cache.insert_unique_unchecked(key, Arc::clone(&entry));
    entry
  }

  // pub fn glyph(&mut self, font_manager: &FontManager, font_handle: FontHandle, character: char, size: u8) -> Arc<GlyphCacheEntry> {
  //   let (is_new, glyph) = self.glyph_allocate(font_manager, font_handle, character, size);
  //   if is_new {
  //     self.glyph_place(&glyph);
  //     self.modified = true;
  //   }
  //   glyph
  // }
}

impl Default for FontTextureManager {
  fn default() -> Self { Self::new() }
}
