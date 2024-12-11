pub mod paint;
pub mod texture;
pub mod text;
pub mod util;

use text::FontManager;
use texture::TextureAtlas;

/// Painter instance, stores textures and fonts needed for rendering
#[derive(Default)]
pub struct PainterInstance {
  pub atlas: TextureAtlas,
  pub fonts: FontManager,
}

impl PainterInstance {
  pub fn new() -> Self {
    Self::default()
  }

  // pub fn atlas(&self) -> &TextureAtlas {
  //   &self.atlas
  // }

  // pub fn atlas_mut(&mut self) -> &mut TextureAtlas {
  //   &mut self.atlas
  // }

  // pub fn fonts(&self) -> &FontManager {
  //   &self.fonts
  // }

  // pub fn fonts_mut(&mut self) -> &mut FontManager {
  //   &mut self.fonts
  // }
}
