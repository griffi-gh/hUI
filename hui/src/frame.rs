use crate::rect::FillColor;

pub mod point;
pub mod layer;

use layer::{FrameLayer, RectLayer};

///XXX: this is not used yet, and also kinda a mess, simplify?
///Maybe limit to a single layer? (aka `Frame` will be just one of the options)
///aka Frame::Rectangle, Frame::NinePatch, ...

/// A frame, which can contain multiple layers
///
/// Use these to construct complex backgrounds
#[derive(Default, Clone)]
pub struct Frame {
  /// Layers of the frame
  layers: Vec<FrameLayer>
}

impl<T: Into<FillColor>> From<T> for Frame {
  fn from(color: T) -> Self {
    let mut frame = Self::default();
    frame.add(RectLayer::from_color(color));
    frame
  }
}

impl Frame {
  pub fn add(&mut self, layer: impl Into<FrameLayer>) -> &mut Self {
    self.layers.push(layer.into());
    self
  }
}
