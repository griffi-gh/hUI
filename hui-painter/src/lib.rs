pub mod paint;
pub mod texture;

use texture::TextureAtlas;

#[derive(Default)]
pub struct Painter {
  pub(crate) atlas: TextureAtlas,
  // ftm: FontTextureManager,
}

impl Painter {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn atlas(&self) -> &TextureAtlas {
    &self.atlas
  }
}
