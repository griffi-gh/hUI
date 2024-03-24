use glam::Vec2;
use crate::draw::UiDrawCommandList;

pub mod point;
mod rect;
pub mod stack;
mod impls;

pub use rect::FrameRect;

pub trait Frame {
  /// Draw the frame at the given position and size
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2);

  /// Check if the frame is guaranteed to be fully opaque and fully cover the parent frame regardless of it's size
  ///
  /// Returns true if the frame:
  /// - Is fully opaque (i.e. `alpha >= 1.0`)
  /// - Completely covers (or exceeds the size of) the frame
  ///
  /// False negatives are acceptable, but false positives ***are not***.\
  /// May be used for optimization purposes
  fn covers_opaque(&self) -> bool { false }
}
