use fontdue::layout::GlyphRasterConfig;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use crate::texture::{SourceTextureFormat, TextureAtlas, TextureHandle};
use super::font::{FontHandle, FontHandleManager, FontId};

/// Maps to the actual texture handle.
struct RasterizedGlyphInternal {
  handle: TextureHandle,
  metrics: fontdue::Metrics,
}

/// Map from raster config to glyph cache item.
///
/// Partitioned by font id in FtM :3
type PartitionKey = HashMap<GlyphRasterConfig, RasterizedGlyphInternal>;

/// Manages glyph cache items in a texture atlas.
pub struct FontTextureManager {
  partition: HashMap<FontId, PartitionKey, BuildNoHashHasher<FontId>>,
}

impl FontTextureManager {
  pub fn new() -> Self {
    Self {
      partition: HashMap::default(),
    }
  }

  /// Drop cached data for the specified font.
  ///
  /// Panics:
  /// - If the font handle is invalid or not initialized.
  /// - If any of the cached items are not found in the texture atlas or became invalid.\
  ///   This may happen if, for example, a different atlas is passed than the one used to allocate the items.
  pub(crate) fn drop_font(&mut self, font: FontHandle, atlas: &mut TextureAtlas) {
    let dump = self.partition.remove(&font.0).expect("Font handle is invalid");
    for (_, item) in dump {
      atlas.remove(item.handle);
    }
  }

  /// Initialize the partition for the specified font.
  ///
  /// Panics:
  /// - If the partition for the font already exists.
  pub(crate) fn init_font(&mut self, font: FontHandle) {
    assert!(!self.partition.contains_key(&font.0), "Font handle already initialized");
    unsafe { self.partition.insert_unique_unchecked(font.0, HashMap::default()) };
  }

  /// Render a glyph and cache it in the texture atlas.
  ///
  /// Panics:
  /// - If the font handle is invalid or not initialized.
  /// - Fuck around and find out, this api is unstable
  pub(crate) fn render_glyph(
    &mut self,
    font_handle: FontHandle,
    fhm_internal: &FontHandleManager,
    config: GlyphRasterConfig,
    atlas: &mut TextureAtlas
  ) -> TextureHandle {
    // Get partiton
    let partition = self.partition.get_mut(&font_handle.0)
      .expect("Font handle is not registered in FtM");

    // Check if glyph is alr cached
    if let Some(item) = partition.get(&config) {
      return item.handle;
    }

    // Get fontdue font from the manager
    let font = &fhm_internal.get_font_repr(font_handle)
      .expect("Font handle is invalid")
      .font;

    // Rasterize the font and copy the texture data
    let (metrics, data) = font.rasterize_config(config);
    let handle = atlas.add_with_data(SourceTextureFormat::A8, &data, metrics.width);

    // Create a texture item struct and insert it into the partition
    let itm = RasterizedGlyphInternal { handle, metrics };
    unsafe { partition.insert_unique_unchecked(config, itm); }

    handle
  }
}

impl Default for FontTextureManager {
  fn default() -> Self {
    Self::new()
  }
}
