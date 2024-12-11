use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

pub(crate) type FontId = u16;

#[derive(Clone, Copy)]
pub struct FontHandle(pub(crate) FontId);

pub(crate) struct FontRepr {
  pub(crate) font: fontdue::Font,
}

pub struct FontHandleManager {
  idc: FontId,
  fonts: HashMap<FontId, FontRepr,BuildNoHashHasher<FontId>>,
}

impl FontHandleManager {
  pub fn new() -> Self {
    Self {
      idc: 0,
      fonts: HashMap::default(),
    }
  }

  /// Add a font to the manager.
  ///
  /// Panics:
  /// - If the font data is invalid.
  pub fn add_font(&mut self, data: &[u8]) -> FontHandle {
    let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).unwrap();
    unsafe { self.fonts.insert_unique_unchecked(self.idc, FontRepr { font }); }
    self.idc += 1;
    FontHandle(self.idc - 1)
  }

  /// Internal function
  ///
  /// Remove and deallocate a font from the manager if the font handle is valid.
  ///
  /// Panics:
  /// - If the font handle is invalid.
  pub(crate) fn remove_font(&mut self, handle: FontHandle) {
    self.fonts.remove(&handle.0).unwrap();
  }

  /// Get the font handle for the specified font.
  pub(crate) fn get_font_repr(&self, handle: FontHandle) -> Option<&FontRepr> {
    self.fonts.get(&handle.0)
  }
}

