pub mod paint;
pub mod texture;

use texture::TextureAtlas;

#[derive(Default)]
pub struct Painter {
  atlas: TextureAtlas,
}

impl Painter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn atlas(&self) -> &TextureAtlas {
    &self.atlas
  }
}
