use glam::{vec2, Vec4};
use crate::{
  background::BackgroundColor,
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  layout::Size,
  measure::Response
};

pub struct Rect {
  pub size: (Size, Size),
  pub color: BackgroundColor,
}

impl Default for Rect {
  fn default() -> Self {
    Self {
      size: (Size::Static(10.), Size::Static(10.)),
      color: Vec4::new(0., 0., 0., 0.5).into(),
    }
  }
}

impl UiElement for Rect {
  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: vec2(
        match self.size.0 {
          Size::Auto => ctx.layout.max_size.x,
          Size::Fraction(percentage) => ctx.layout.max_size.x * percentage,
          Size::Static(pixels) => pixels,
        },
        match self.size.1 {
          Size::Auto => ctx.layout.max_size.y,
          Size::Fraction(percentage) => ctx.layout.max_size.y * percentage,
          Size::Static(pixels) => pixels,
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
