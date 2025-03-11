#![no_std]
#[macro_use]
extern crate alloc;

pub mod paint;
pub mod texture;
pub mod text;
pub mod util;
pub mod backend;
pub mod presentation;

use text::FontManager;
use texture::TextureAtlas;

/// Painter instance, stores textures and fonts needed for rendering
#[derive(Default)]
pub struct PainterInstance {
  pub(crate) textures: TextureAtlas,
  pub(crate) fonts: FontManager,
}

impl PainterInstance {
  pub fn new() -> Self {
    Self::default()
  }

  /// Get an immutable reference to the texture atlas
  pub fn textures(&self) -> &TextureAtlas {
    &self.textures
  }

  /// Get a mutable reference to the texture atlas
  pub fn textures_mut(&mut self) -> &mut TextureAtlas {
    &mut self.textures
  }

  /// Get an immutable reference to the font manager
  pub fn fonts(&self) -> &FontManager {
    &self.fonts
  }

  /// Get a mutable reference to the font manager
  pub fn fonts_mut(&mut self) -> &mut FontManager {
    &mut self.fonts
  }
}
