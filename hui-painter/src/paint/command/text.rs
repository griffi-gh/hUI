use std::borrow::Cow;
use fontdue::layout::{CoordinateSystem, Layout};
use glam::{vec2, Vec2};
use crate::{
  paint::{
    buffer::PaintBuffer,
    command::PaintCommand,
  }, text::FontHandle, PainterInstance
};

use super::Measurable;

// TODO align, multichunk etc

pub struct TextChunk {
  pub text: Cow<'static, str>,
  pub font: FontHandle,
  pub size: f32,
}

pub struct PaintText {
  // TODO multiple text chunks
  pub text: TextChunk,
}

impl PaintText {
  pub fn new(text: impl Into<Cow<'static, str>>, font: FontHandle, size: f32) -> Self {
    Self {
      text: TextChunk {
        text: text.into(),
        font,
        size,
      }
    }
  }

  fn build_font_array<'a>(&self, ctx: &'a PainterInstance) -> Vec<&'a fontdue::Font> {
    let font = ctx.fonts.get_fontdue_font(self.text.font)
      .expect("FontHandle is invalid");
    vec![&font]
  }

  fn build_layout(&self, font_array: &[&fontdue::Font]) -> Layout {
    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.append(
      &font_array,
      &fontdue::layout::TextStyle::new(
        &self.text.text,
        self.text.size,
        0
      )
    );
    layout
  }
}

impl PaintCommand for PaintText {
  fn pre_paint(&self, ctx: &mut PainterInstance) {
    let font_array = self.build_font_array(ctx);
    let layout = self.build_layout(&font_array);

    for glyph in layout.glyphs() {
      ctx.fonts.render_glyph(&mut ctx.atlas, self.text.font, glyph.key);
    }
  }

  fn paint(&self, ctx: &mut PainterInstance, into: &mut PaintBuffer) {
    // let font_array = self.build_font_array(ctx);
    // let layout = self.build_layout(&font_array);

    // for glyph in layout.glyphs() {
    //   let config = GlyphRasterConfig {
    //     glyph_index: glyph.font_index
    //   };
    //   let glyph_raster = ctx.fonts().render_glyph(atlas, font, config);
    // }

    todo!()
  }
}

impl Measurable for PaintText {
  fn size(&self, ctx: &PainterInstance) -> Vec2 {
    let font_array = self.build_font_array(ctx);
    let layout = self.build_layout(&font_array);

    let width = layout.lines().map(|lines| {
      lines.iter().fold(0.0_f32, |acc, x| {
        let glyph = layout.glyphs().get(x.glyph_end).unwrap();
        acc.max(glyph.x + glyph.width as f32)
      })
    }).unwrap_or(0.);
    let height = layout.height();

    vec2(width, height)
  }
}
