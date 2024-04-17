//! modular procedural background system

use crate::{draw::UiDrawCommandList, rect::Rect};

pub mod point;
mod rect;
pub mod stack;
pub mod nine_patch;
mod impls;

pub use rect::RectFrame;

/// Trait for a drawable frame
pub trait Frame {
  /// Draw the frame at the given rect's position and size
  fn draw(&self, draw: &mut UiDrawCommandList, rect: Rect);

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
