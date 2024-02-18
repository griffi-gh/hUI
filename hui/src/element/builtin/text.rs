use std::borrow::Cow;
use glam::{vec2, Vec4};
use crate::{
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response,
  text::{FontHandle, BUILTIN_FONT},
  UiSize
};

pub struct Text {
  pub text: Cow<'static, str>,
  pub size: (UiSize, UiSize),
  pub color: Vec4,
  pub font: FontHandle,
  pub text_size: u8,
}

impl Default for Text {
  fn default() -> Self {
    Self {
      text: "".into(),
      size: (UiSize::Auto, UiSize::Auto),
      color: Vec4::new(1., 1., 1., 1.),
      font: BUILTIN_FONT,
      text_size: 16,
    }
  }
}

impl UiElement for Text {
  fn measure(&self, ctx: MeasureContext) -> Response {
    let mut size = (0., 0.);
    if matches!(self.size.0, UiSize::Auto) || matches!(self.size.1, UiSize::Auto) {
      let res = ctx.text_measure.measure(self.font, self.text_size, &self.text);
      size.0 = res.max_width;
      size.1 = res.height;
    }
    Response {
      size: vec2(
        match self.size.0 {
          UiSize::Auto => size.0,
          UiSize::Percentage(percentage) => ctx.layout.max_size.x * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
        match self.size.1 {
          UiSize::Auto => size.1,
          UiSize::Percentage(percentage) => ctx.layout.max_size.y * percentage,
          UiSize::Pixels(pixels) => pixels,
        },
      ),
      hints: Default::default(),
      user_data: None
    }
  }

  fn process(&self, ctx: ProcessContext) {
    ctx.draw.add(UiDrawCommand::Text {
      text: self.text.clone(),
      position: ctx.layout.position,
      size: self.text_size,
      color: self.color,
      font: self.font
    });
  }
}
