use glam::Vec2;
use crate::draw::UiDrawCommandList;

pub mod point;
mod rect;
pub mod stack;

pub use rect::FrameRect;

pub trait Frame {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2);
}
