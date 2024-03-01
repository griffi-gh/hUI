use std::borrow::Cow;
use derive_setters::Setters;
use glam::{vec2, Vec4};
use crate::{
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{Size, Size2d},
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
  #[setters(into)]
  pub size: Size2d,
  #[setters(into)]
  pub color: Vec4,
  #[setters(into)]
  pub font: Option<FontHandle>,
  pub text_size: u16,
}

impl Default for Text {
  fn default() -> Self {
    Self {
      text: "".into(),
      size: (Size::Auto, Size::Auto).into(),
      color: Vec4::new(1., 1., 1., 1.),
      font: None,
      text_size: 16,
    }
  }
}

impl Text {
  fn font(&self, f: FontHandle) -> FontHandle {
    self.font.unwrap_or(f)
  }
}

impl UiElement for Text {
  fn measure(&self, ctx: MeasureContext) -> Response {
    let mut size = (0., 0.);
    if matches!(self.size.width, Size::Auto) || matches!(self.size.height, Size::Auto) {
      //TODO optimized measure if only one of the sizes is auto
      let res = ctx.text_measure.measure(self.font(ctx.current_font), self.text_size, &self.text);
      size.0 = res.max_width;
      size.1 = res.height;
    }
    Response {
      size: vec2(
        match self.size.width {
          Size::Auto => size.0,
          Size::Fraction(percentage) => ctx.layout.max_size.x * percentage,
          Size::Static(pixels) => pixels,
        },
        match self.size.height {
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
    if self.text.is_empty() || self.color.w == 0. {
      return
    }
    ctx.draw.add(UiDrawCommand::Text {
      text: self.text.clone(),
      position: ctx.layout.position,
      size: self.text_size,
      color: self.color,
      font: self.font(ctx.current_font),
    });
  }
}
