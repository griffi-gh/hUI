use crate::{RawDataLoader, TextureData, TextureLoader};

pub struct ImageLoader<T: RawDataLoader>(pub T);

impl<T: RawDataLoader> From<T> for ImageLoader<T> {
  fn from(loader: T) -> Self {
    Self(loader)
  }
}

impl<T: RawDataLoader> TextureLoader for ImageLoader<T> {
  fn load(&self) -> TextureData {
    let data = self.0.load();
    todo!()
  }
}