use glam::{vec2, Vec4, vec4};
use crate::{
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response,
  UiSize
};

#[derive(Debug, Clone, Copy)]
pub struct ProgressBar {
  pub size: (UiSize, UiSize),
  pub value: f32,
  pub color_foreground: Vec4,
  pub color_background: Vec4,
}

impl Default for ProgressBar {
  fn default() -> Self {
    Self {
      size: (UiSize::Auto, UiSize::Auto),
      value: 0.,
      color_foreground: vec4(0.0, 0.0, 1.0, 1.0),
      color_background: vec4(0.0, 0.0, 0.0, 1.0),
    }
  }
}

const BAR_HEIGHT: f32 = 20.0;

impl UiElement for ProgressBar {
  fn name(&self) -> &'static str { "Progress bar" }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: vec2(
        match self.size.0 {
          UiSize::Auto => ctx.layout.max_size.x.max(300.),
          UiSize::Fraction(p) => ctx.layout.max_size.x * p,
          UiSize::Static(p) => p,
        },
        match self.size.1 {
          UiSize::Auto => BAR_HEIGHT,
          UiSize::Fraction(p) => ctx.layout.max_size.y * p,
          UiSize::Static(p) => p,
        }
      ),
      hints: Default::default(),
      user_data: None,
    }
  }

  fn process(&self, ctx: ProcessContext) {
    let value = self.value.clamp(0., 1.);
    if value < 1. {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color: self.color_background
      });
    }
    if value > 0. {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size * vec2(value, 1.0),
        color: self.color_foreground
      });
    }
  }
}
