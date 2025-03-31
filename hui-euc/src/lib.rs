#![cfg_attr(not(feature = "std"), no_std)]

use hui_painter::backend::BackendData;

mod pipeline;
use pipeline::UiPipeline;

pub struct EucUiRenderer {
  pipeline: UiPipeline,
  // output: Buffer2d<<UiPipeline as Pipeline>::Pixel>,
}

impl EucUiRenderer {
  pub fn new() -> Self {
    Self {
      pipeline: UiPipeline::default(),
    }
  }

  pub fn update(&mut self, data: &BackendData) {

  }
}

impl Default for EucUiRenderer {
  fn default() -> Self {
    Self::new()
  }
}

