use std::borrow::Cow;
use derive_setters::Setters;
use glam::{vec2, Vec4};
use crate::{
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  layout::Size,
  measure::Response,
  text::FontHandle,
};


//TODO: text fit
// pub enum TextSize {
//   FitToWidthRatio(f32),
//   FitToHeightRatio(f32),
//   Constant(u8),
// }

#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct Text {
  #[setters(into)]
  pub text: Cow<'static, str>,
  pub size: (Size, Size),
  pub color: Vec4,
  pub font: FontHandle,
  pub text_size: u16,
}

impl Default for Text {
  fn default() -> Self {
    Self {
      text: "".into(),
      size: (Size::Auto, Size::Auto),
      color: Vec4::new(1., 1., 1., 1.),
      font: FontHandle::default(),
      text_size: 16,
    }
  }
}

impl UiElement for Text {
  fn measure(&self, ctx: MeasureContext) -> Response {
    let mut size = (0., 0.);
    if matches!(self.size.0, Size::Auto) || matches!(self.size.1, Size::Auto) {
      let res = ctx.text_measure.measure(self.font, self.text_size, &self.text);
      size.0 = res.max_width;
      size.1 = res.height;
    }
    Response {
      size: vec2(
        match self.size.0 {
          Size::Auto => size.0,
          Size::Fraction(percentage) => ctx.layout.max_size.x * percentage,
          Size::Static(pixels) => pixels,
        },
        match self.size.1 {
          Size::Auto => size.1,
          Size::Fraction(percentage) => ctx.layout.max_size.y * percentage,
          Size::Static(pixels) => pixels,
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
