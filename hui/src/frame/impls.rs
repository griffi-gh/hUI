use glam::{Vec2, Vec3, Vec4};
use super::Frame;
use crate::{
  color,
  draw::{ImageHandle, UiDrawCommand, UiDrawCommandList},
  rect::{Corners, FillColor},
};

impl Frame for ImageHandle {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    draw.add(UiDrawCommand::Rectangle {
      position,
      size: parent_size,
      color: color::WHITE.into(),
      texture: Some(*self),
      rounded_corners: None,
    })
  }
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

// impl for various types resembling colors

// Corners (RGBA):

impl Frame for Corners<Vec4> {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for (Vec4, Vec4, Vec4, Vec4) {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for ((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32)) {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for [[f32; 4]; 4] {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

// Corners (RGB):

impl Frame for Corners<Vec3> {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for (Vec3, Vec3, Vec3, Vec3) {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for [[f32; 3]; 4] {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

// RGBA:

impl Frame for Vec4 {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for (f32, f32, f32, f32) {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for [f32; 4] {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

// RGB:

impl Frame for Vec3 {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for (f32, f32, f32) {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}

impl Frame for [f32; 3] {
  fn draw(&self, draw: &mut UiDrawCommandList, position: Vec2, parent_size: Vec2) {
    FillColor::from(*self).draw(draw, position, parent_size)
  }
}
