//! allows stacking two frames on top of each other

use glam::Vec2;
use crate::draw::UiDrawCommandList;
use super::Frame;

/// A frame that draws two frames on top of each other
pub struct FrameStack(pub Box<dyn Frame>, pub Box<dyn Frame>);

impl Frame for FrameStack {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    self.0.draw(draw, position, parent_size);
    self.1.draw(draw, position, parent_size);
  }

  fn covers_opaque(&self) -> bool {
    self.0.covers_opaque() ||
    self.1.covers_opaque()
  }
}

pub trait FrameStackExt: Frame {
  /// Stack another frame on top of this one
  fn stack(self, other: impl Frame + 'static) -> FrameStack;
}

impl<T: Frame + 'static> FrameStackExt for T {
  fn stack(self, other: impl Frame + 'static) -> FrameStack {
    FrameStack(Box::new(self), Box::new(other))
  }
}
