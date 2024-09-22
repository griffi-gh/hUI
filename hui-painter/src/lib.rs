pub mod paint;
pub mod texture;
pub mod text;

use text::FontManager;
use texture::TextureAtlas;

#[derive(Default)]
pub struct Painter {
  pub atlas: TextureAtlas,
  pub fonts: FontManager,
}

impl Painter {
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
