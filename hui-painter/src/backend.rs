use crate::{
  presentation::{Presentatation, PresentatationBackendData},
  texture::TextureAtlasBackendData,
  PainterInstance,
};

#[derive(Clone, Copy)]
pub struct BackendData<'a> {
  pub presentation: PresentatationBackendData<'a>,
  pub atlas: TextureAtlasBackendData<'a>,
}

impl PainterInstance {
  pub fn backend_data<'a>(&'a self, presentation: &'a Presentatation) -> BackendData<'a> {
    BackendData {
      presentation: presentation.backend_data(),
      atlas: self.textures.backend_data(),
    }
  }
}


