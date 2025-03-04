//! allows stacking two frames on top of each other

use hui_painter::paint::command::PaintList;
use crate::rect::Rect;
use super::Frame;

/// A frame that draws two frames on top of each other
pub struct FrameStack(pub Box<dyn Frame>, pub Box<dyn Frame>);

impl Frame for FrameStack {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    self.0.draw(draw, rect);
    self.1.draw(draw, rect);
  }

  fn covers_opaque(&self) -> bool {
    self.0.covers_opaque() ||
    self.1.covers_opaque()
  }
}

pub trait FrameStackExt: Frame {
  /// Stack another frame on top of this one
  fn stack(self, other: impl Frame + 'static) -> FrameStack;

  /// Stack another frame below this one
  fn stack_below(self, other: impl Frame + 'static) -> FrameStack;
}

impl<T: Frame + 'static> FrameStackExt for T {
  fn stack(self, other: impl Frame + 'static) -> FrameStack {
    FrameStack(Box::new(self), Box::new(other))
  }

  fn stack_below(self, other: impl Frame + 'static) -> FrameStack {
    FrameStack(Box::new(other), Box::new(self))
  }
}
