use core::hash::{Hash, Hasher};
use alloc::{borrow::Cow, vec::Vec};
use fontdue::layout::{CoordinateSystem, Layout};
use glam::{vec2, Vec4};
use hui_shared::rect::Rect;
use crate::{
  paint::{
    buffer::{PaintBuffer, Vertex},
    command::PaintCommand,
  },
  text::FontHandle,
  util::hash_vec4,
  PainterInstance,
};

// TODO align, multichunk etc

pub struct TextChunk {
  pub text: Cow<'static, str>,
  pub font: FontHandle,
  pub size: f32,
  // TODO support FillColor for text color (should it apply to the whole text or per character?)
  pub color: Vec4,
}

pub struct PaintText {
  // TODO multiple text chunks
  pub text: TextChunk,
}

impl PaintText {
  pub fn new(text: impl Into<Cow<'static, str>>, font: FontHandle, size: f32, color: impl Into<Vec4>) -> Self {
    Self {
      text: TextChunk {
        text: text.into(),
        font,
        size,
        color: color.into(),
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
      font_array,
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
    if self.text.text.trim().is_empty() {
      return
    }

    let font_array = self.build_font_array(ctx);
    let layout = self.build_layout(&font_array);

    for glyph in layout.glyphs() {
      if !glyph.char_data.rasterize() {
        continue;
      }
      ctx.fonts.render_glyph(&mut ctx.textures, self.text.font, glyph.key);
    }
  }

  fn paint(&self, ctx: &mut PainterInstance, into: &mut PaintBuffer) {
    if self.text.text.trim().is_empty() {
      return
    }

    let font_array = self.build_font_array(ctx);
    let layout = self.build_layout(&font_array);

    let glyphs = layout.glyphs();

    for glyph in glyphs {
      if !glyph.char_data.rasterize() {
        continue
      }

      // let fontdue_font = font_array[layout_glyph.font_index];
      let font_handle = self.text.font; // TODO use font_index here

      let vidx = into.vertices.len() as u32;
      let glyph_texture = ctx.fonts.render_glyph(&mut ctx.textures, font_handle, glyph.key);
      let uv = ctx.textures.get_uv(glyph_texture).unwrap();

      into.indices.extend([vidx, vidx + 1, vidx + 2, vidx, vidx + 2, vidx + 3]);
      into.vertices.extend([
        Vertex {
          position: vec2(glyph.x, glyph.y).round(),
          color: self.text.color,
          uv: uv.top_left,
        },
        Vertex {
          position: vec2(glyph.x + glyph_texture.size().x as f32, glyph.y).round().round(),
          color: self.text.color,
          uv: uv.top_right,
        },
        Vertex {
          position: vec2(glyph.x + glyph_texture.size().x as f32, glyph.y + glyph_texture.size().y as f32).round(),
          color: self.text.color,
          uv: uv.bottom_right,
        },
        Vertex {
          position: vec2(glyph.x, glyph.y + glyph_texture.size().y as f32).round(),
          color: self.text.color,
          uv: uv.bottom_left,
        },
      ]);
    }

    // for glyph in layout.glyphs() {
    //   let config = GlyphRasterConfig {
    //     glyph_index: glyph.font_index as u16,
    //   };
    //   let glyph_raster = ctx.fonts().render_glyph(atlas, font, config);
    // }
  }

  fn bounds(&self, ctx: &PainterInstance) -> Rect {
    let font_array = self.build_font_array(ctx);
    let layout = self.build_layout(&font_array);

    let width = layout.lines().map(|lines| {
      lines.iter().fold(0.0_f32, |acc, x| {
        let glyph = layout.glyphs().get(x.glyph_end).unwrap();
        acc.max(glyph.x + glyph.width as f32)
      })
    }).unwrap_or(0.);
    let height = layout.height();

    Rect {
      position: vec2(0., 0.),
      size: vec2(width, height),
    }
  }

  fn cache_hash(&self) -> u64 {
    let mut hasher = rustc_hash::FxHasher::default();

    // cache font/size/color
    self.text.font.hash(&mut hasher);
    hasher.write_u32(self.text.size.to_bits());
    hash_vec4(&mut hasher, self.text.color);

    // cache text content
    match self.text.text {
      Cow::Owned(ref s) => hasher.write(s.as_bytes()),
      Cow::Borrowed(s) => {
        // since the lifetime is 'static, the str is guaranteed to never change
        // so we can safely compare the ptr + len instead of the content
        hasher.write_usize(s.as_ptr() as usize);
        hasher.write_usize(s.len());
      }
    }

    hasher.finish()
  }
}
