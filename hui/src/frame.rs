use glam::Vec2;
use crate::{draw::{UiDrawCommand, UiDrawCommandList}, rect::FillColor};

pub mod point;
mod rect;
pub mod stack;

pub use rect::FrameRect;

pub trait Frame {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2);
}

impl Frame for FillColor {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    draw.add(UiDrawCommand::Rectangle {
      position,
      size: parent_size,
      color: self.corners(),
      texture: None,
      rounded_corners: None,
    })
  }
}

// impl<T: Into<FillColor> + Clone> Frame for T {
//   fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
//     let color: FillColor = self.clone().into();
//     draw.add(UiDrawCommand::Rectangle {
//       position,
//       size: parent_size,
//       color: color.corners(),
//       texture: None,
//       rounded_corners: None,
//     })
//   }
// }
