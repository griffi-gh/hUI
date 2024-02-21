use fontdue::Font;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct FontHandle(pub(crate) usize);

#[cfg(feature = "builtin_font")]
pub const BUILTIN_FONT: FontHandle = FontHandle(0);

pub(crate) const DEFAULT_FONT: FontHandle = {
  #[cfg(feature = "builtin_font")] { BUILTIN_FONT }
  #[cfg(not(feature = "builtin_font"))] { FontHandle(usize::MAX) }
};

#[cfg(feature = "builtin_font")]
const BUILTIN_FONT_DATA: &[u8] = include_bytes!("../../assets/font/ProggyTiny.ttf");

pub struct FontManager {
  fonts: Vec<Font>,
}

impl FontManager {
  pub fn new() -> Self {
    let mut this = Self {
      fonts: Vec::new(),
    };
    #[cfg(feature = "builtin_font")]
    {
      let font = Font::from_bytes(
        BUILTIN_FONT_DATA,
        fontdue::FontSettings::default()
      ).unwrap();
      this.add_font(font);
    };
    this
  }

  /// Add a (fontdue) font to the renderer.
  pub fn add_font(&mut self, font: Font) -> FontHandle {
    self.fonts.push(font);
    FontHandle(self.fonts.len() - 1)
  }

  pub fn get(&self, handle: FontHandle) -> Option<&Font> {
    self.fonts.get(handle.0)
  }
}

impl Default for FontManager {
  fn default() -> Self {
    Self::new()
  }
}
