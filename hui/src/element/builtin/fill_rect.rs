//! Simple filled rectangle with the specified size, background and corner radius

use derive_setters::Setters;
use glam::{vec2, Vec4};
use crate::{
  background::RectBackground,
  draw::{UiDrawCommand, RoundedCorners},
  element::{UiElement, MeasureContext, ProcessContext},
  layout::{Size, Size2d},
  measure::Response,
  rectangle::Corners,
  size,
};

/// Simple filled rectangle with the specified size, background, and corner radius
#[derive(Debug, Clone, Copy, Setters)]
#[setters(prefix = "with_")]
pub struct FillRect {
  /// Size of the rectangle
  #[setters(into)]
  pub size: Size2d,

  /// Background color of the rectangle
  #[setters(into)]
  pub background: RectBackground,

  /// Corner radius of the rectangle
  #[setters(into)]
  pub corner_radius: Corners<f32>,
}

impl Default for FillRect {
  fn default() -> Self {
    Self {
      size: size!(10, 10),
      background: Vec4::new(0., 0., 0., 0.5).into(),
      corner_radius: Corners::all(0.),
    }
  }
}

impl UiElement for FillRect {
  fn name(&self) -> &'static str {
    "FillRect"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: vec2(
        match self.size.width {
          Size::Auto => ctx.layout.max_size.x,
          Size::Fraction(percentage) => ctx.layout.max_size.x * percentage,
          Size::Static(pixels) => pixels,
        },
        match self.size.height {
          Size::Auto => ctx.layout.max_size.y,
          Size::Fraction(percentage) => ctx.layout.max_size.y * percentage,
          Size::Static(pixels) => pixels,
        },
      ),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if !self.background.is_transparent() {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color: self.background.corners().unwrap(),
        texture: None,
        rounded_corners: (self.corner_radius.max_f32() > 0.).then_some({
          RoundedCorners::from_radius(self.corner_radius)
        }),
      });
    }
  }
}
