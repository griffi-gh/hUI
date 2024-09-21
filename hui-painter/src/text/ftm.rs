use fontdue::layout::GlyphRasterConfig;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use crate::texture::{TextureAtlas, TextureHandle};

type FontId = u16;

#[derive(Clone, Copy)]
pub struct FontHandle(FontId);

/// Maps to the actual texture handle.
struct GlyphCacheItem {
  handle: TextureHandle,
}

/// Map from raster config to glyph cache item.
///
/// Partitioned by font id in FtM :3
type PartitionKey = HashMap<GlyphRasterConfig, GlyphCacheItem>;

/// Manages glyph cache items in a texture atlas.
pub struct FontTextureManager {
  partition: HashMap<FontId, PartitionKey, BuildNoHashHasher<FontId>>,
}

impl FontTextureManager {
  /// Drop cached data for the specified font.
  ///
  /// Panics:
  /// - If the font handle is invalid.
  /// - If any of the cached items are not found in the texture atlas or became invalid.\
  ///   This may happen if, for example, a different atlas is passed than the one used to allocate the items.
  fn drop_font(&mut self, font: FontHandle, atlas: &mut TextureAtlas) {
    let dump = self.partition.remove(&font.0).expect("Font handle is invalid");
    for (_, item) in dump {
      atlas.deallocate(item.handle);
    }
  }
}

