use glam::{vec2, Vec4};
use crate::{
  background::BackgroundColor,
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  layout::UiSize,
  measure::Response
};

pub struct Rect {
  pub size: (UiSize, UiSize),
  pub color: BackgroundColor,
}

impl Default for Rect {
  fn default() -> Self {
    Self {
      size: (UiSize::Static(10.), UiSize::Static(10.)),
      color: Vec4::new(0., 0., 0., 0.5).into(),
    }
  }
}

impl UiElement for Rect {
  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: vec2(
        match self.size.0 {
          UiSize::Auto => ctx.layout.max_size.x,
          UiSize::Fraction(percentage) => ctx.layout.max_size.x * percentage,
          UiSize::Static(pixels) => pixels,
        },
        match self.size.1 {
          UiSize::Auto => ctx.layout.max_size.y,
          UiSize::Fraction(percentage) => ctx.layout.max_size.y * percentage,
          UiSize::Static(pixels) => pixels,
        },
      ),
      hints: Default::default(),
      user_data: None
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if !self.color.is_transparent() {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color: self.color.corners().unwrap(),
        texture: None,
        rounded_corners: None,
      });
    }
  }
}
