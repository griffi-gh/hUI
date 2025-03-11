//! simple text element, renders a string of text

use std::borrow::Cow;
use derive_setters::Setters;
use glam::{Affine2, Vec4};
use hui_painter::{
  paint::command::{text::{PaintText, TextChunk}, PaintCommand, PaintTransform},
  text::FontHandle,
};
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{compute_size, Size, Size2d},
  measure::Response,
};

//TODO: text fit
// pub enum TextSize {
//   FitToWidthRatio(f32),
//   FitToHeightRatio(f32),
//   Constant(u8),
// }

/// Simple text element, renders a string of text
#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct Text {
  /// Text to render
  #[setters(into)]
  pub text: Cow<'static, str>,

  /// Size of the text element
  #[setters(into)]
  pub size: Size2d,

  /// Color of the text
  #[setters(into)]
  pub color: Vec4,

  /// Font to use for rendering the text\
  /// If set to `None` either currently selected font or the default font will be used
  #[setters(into)]
  pub font: Option<FontHandle>,

  /// Size of the text, in points (these are not pixels)
  pub text_size: f32,
}

impl Default for Text {
  fn default() -> Self {
    Self {
      text: "".into(),
      size: (Size::Auto, Size::Auto).into(),
      color: Vec4::new(1., 1., 1., 1.),
      font: None,
      text_size: 16.,
    }
  }
}

impl Text {
  pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
    Self {
      text: text.into(),
      ..Default::default()
    }
  }

  fn font(&self, f: FontHandle) -> FontHandle {
    self.font.unwrap_or(f)
  }
}

impl Text {
  fn paint_cmd(&self, current_font: FontHandle) -> PaintText {
    PaintText {
      text: TextChunk {
        text: self.text.clone(),
        font: self.font.unwrap_or(current_font),
        size: self.text_size as f32,
        color: self.color.into(),
      }
    }
  }
}

impl UiElement for Text {
  fn name(&self) -> &'static str {
    "text"
  }

  fn size(&self) -> Option<Size2d> {
    Some(self.size)
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    let mut size = (0., 0.);
    if matches!(self.size.width, Size::Auto) || matches!(self.size.height, Size::Auto) {
      //TODO optimized measure if only one of the sizes is auto
      // let res = ctx.text_measure.measure(self.font(ctx.current_font), self.text_size, &self.text);
      // size.0 = res.max_width;
      // size.1 = res.height;
      let cmd = self.paint_cmd(ctx.current_font);
      let cmd_size = cmd.bounds(ctx.painter).size;
      size.0 = cmd_size.x;
      size.1 = cmd_size.y;
    }
    Response {
      size: compute_size(ctx.layout, self.size, size.into()),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if self.text.is_empty() || self.color.w == 0. {
      return
    }
    ctx.paint_target.add(PaintTransform {
      transform: Affine2::from_translation(ctx.layout.position),
      child: self.paint_cmd(ctx.current_font),
    });
  }
}
