pub mod point;
pub mod layer;

use layer::{FrameLayer, RectFrame};

///XXX: this is not used yet, and also kinda a mess, simplify?
///Maybe limit to a single layer? (aka `Frame` will be just one of the options)
///Because currently, this is just a duplicate of the dormal draw command system, but with a different name...
///Then, there's no need for the positioning stuff too, which is a bit overkill and is kinda code duplication too!
///aka Frame::Rectangle, Frame::NinePatch, ...

/// A frame, which can contain multiple layers
///
/// Use these to construct complex backgrounds
#[derive(Default, Clone)]
pub struct Frame {
  /// Layers of the frame
  layers: Vec<FrameLayer>
}

impl<T: Into<FrameLayer>> From<T> for Frame {
  fn from(layer: T) -> Self {
    let mut frame = Self::default();
    frame.add(layer.into());
    frame
  }
}

impl Frame {
  /// Get the layer with the given index
  #[inline]
  pub fn layer(&self, index: usize) -> Option<&FrameLayer> {
    self.layers.get(index)
  }

  /// Get a mutable reference to the layer with the given index
  #[inline]
  pub fn layer_mut(&mut self, index: usize) -> Option<&mut FrameLayer> {
    self.layers.get_mut(index)
  }

  /// Add a layer to the frame
  #[inline]
  pub fn add(&mut self, layer: impl Into<FrameLayer>) -> &mut Self {
    self.layers.push(layer.into());
    self
  }

  /// Add a layer to the back of the frame
  #[inline]
  pub fn add_back(&mut self, layer: impl Into<FrameLayer>) -> &mut Self {
    self.layers.insert(0, layer.into());
    self
  }

  #[inline]
  pub fn finish(&mut self) -> Self {
    self.clone()
  }
}
