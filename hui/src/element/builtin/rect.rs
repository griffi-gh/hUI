use glam::{vec2, Vec4};
use crate::{
  draw::{UiDrawCommand, UiDrawCommands},
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response,
  state::StateRepo,
  LayoutInfo, UiSize
};

pub struct Rect {
  pub size: (UiSize, UiSize),
  pub color: Option<Vec4>,
}

impl Default for Rect {
  fn default() -> Self {
    Self {
      size: (UiSize::Pixels(10.), UiSize::Pixels(10.)),
      color: Some(Vec4::new(0., 0., 0., 0.5)),
    }
  }
}

impl UiElement for Rect {
  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: vec2(
        match self.size.0 {
          UiSize::Auto => ctx.layout.max_size.x,
          UiSize::Percentage(percentage) => ctx.layout.max_size.x * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
        match self.size.1 {
          UiSize::Auto => ctx.layout.max_size.y,
          UiSize::Percentage(percentage) => ctx.layout.max_size.y * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
      ),
      hints: Default::default(),
      user_data: None
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if let Some(color) = self.color {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color,
      });
    }
  }
}
