use std::{borrow::Cow, sync::Arc};
use fontdue::layout::{CoordinateSystem, Layout};
use crate::{
  Painter,
  paint::{
    buffer::PaintBuffer,
    command::PaintCommand,
  },
};

pub struct TextChunk {
  pub text: Cow<'static, str>,
  pub font: (),
  pub size: f32,
}

pub struct PaintText {
  // TODO multiple text chunks
  pub text: TextChunk,
}

impl PaintText {
  pub fn new(text: impl Into<Cow<'static, str>>, size: f32) -> Self {
    Self {
      text: TextChunk {
        text: text.into(),
        font: todo!(),
        size,
      }
    }
  }
}

impl PaintCommand for PaintText {
  fn paint(&self, ctx: &mut Painter, into: &mut PaintBuffer) {
    // let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    // layout.append(
    //   &[text_renderer.internal_font(*font_handle)],
    //   &TextStyle::new(text, *size as f32, 0)
    // );

    todo!()
  }
}
