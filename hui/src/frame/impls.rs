use glam::{Affine2, Vec3, Vec4};
use hui_painter::{paint::command::{PaintList, PaintRectangle, PaintTransform}, texture::TextureHandle};
use super::Frame;
use crate::{
  color,
  rect::{Rect, Corners, FillColor},
};

impl Frame for TextureHandle {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    draw.add(PaintTransform {
      transform: Affine2::from_translation(rect.position),
      child: PaintRectangle {
        size: rect.size,
        color: color::WHITE.into(),
        texture: Some(*self),
        ..Default::default()
      },
    });
  }

  fn covers_opaque(&self) -> bool {
    false
  }
}

impl Frame for FillColor {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    if self.is_transparent() {
      return
    }
    draw.add(PaintTransform {
      transform: Affine2::from_translation(rect.position),
      child: PaintRectangle {
        size: rect.size,
        color: *self,
        ..Default::default()
      },
    })
  }

  fn covers_opaque(&self) -> bool {
    self.is_opaque()
  }
}

// impl for various types resembling colors

// Corners (RGBA):

impl Frame for Corners<Vec4> {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

impl Frame for (Vec4, Vec4, Vec4, Vec4) {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

impl Frame for ((f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32), (f32, f32, f32, f32)) {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

impl Frame for [[f32; 4]; 4] {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

// Corners (RGB):

impl Frame for Corners<Vec3> {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}

impl Frame for (Vec3, Vec3, Vec3, Vec3) {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}

impl Frame for ((f32, f32, f32), (f32, f32, f32), (f32, f32, f32), (f32, f32, f32)) {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}

impl Frame for [[f32; 3]; 4] {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}

// RGBA:

impl Frame for Vec4 {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

impl Frame for (f32, f32, f32, f32) {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

impl Frame for [f32; 4] {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    FillColor::from(*self).is_opaque()
  }
}

// RGB:

impl Frame for Vec3 {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}

impl Frame for (f32, f32, f32) {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}

impl Frame for [f32; 3] {
  fn draw(&self, draw: &mut PaintList, rect: Rect) {
    FillColor::from(*self).draw(draw, rect)
  }
  fn covers_opaque(&self) -> bool {
    true
  }
}
