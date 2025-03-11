use hui_painter::texture::{SourceTextureFormat, TextureAtlas, TextureHandle};

pub mod loaders {
  #[cfg(feature = "loader-file")]
  mod file;
  #[cfg(feature = "loader-file")]
  pub use file::FileLoader;

  #[cfg(feature = "loader-image")]
  mod image;
  #[cfg(feature = "loader-image")]
  pub use image::ImageLoader;
}

pub trait RawDataLoader {
  /// Syncronously load the raw data from the source
  fn load(&self) -> Vec<u8>;
}

pub struct TextureData {
  /// Texture data in the RGBA8 format
  pub data: Vec<u8>,

  /// Texture width in pixel
  pub width: usize,
}

pub trait TextureLoader {
  /// Syncronously load the texture data
  fn load(&self) -> TextureData;
}

pub trait AtlasLoadersExt {
  fn add_with_loader(&mut self, loader: impl TextureLoader) -> TextureHandle;
}

impl AtlasLoadersExt for TextureAtlas {
  fn add_with_loader(&mut self, loader: impl TextureLoader) -> TextureHandle {
    let data = loader.load();
    self.add_with_data(SourceTextureFormat::RGBA8, &data.data, data.width)
  }
}

