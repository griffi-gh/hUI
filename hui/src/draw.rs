//! Stuff related to tesselation and UI rendering.

use crate::{
  rectangle::Corners,
  text::{FontHandle, TextRenderer}
};

pub(crate) mod atlas;
use atlas::TextureAtlasManager;
pub use atlas::{TextureHandle, TextureAtlasMeta};

mod corner_radius;
pub use corner_radius::RoundedCorners;

use std::borrow::Cow;
use fontdue::layout::{Layout, CoordinateSystem, TextStyle};
use glam::{Vec2, Vec4, vec2};

/// Available draw commands
/// - Rectangle: Filled, colored rectangle, with optional rounded corners
/// - Circle: Simple filled, colored circle
/// - Text: Draw text using the specified font, size, color, and position
#[derive(Clone, Debug, PartialEq)]
pub enum UiDrawCommand {
  ///Filled, colored rectangle
  Rectangle {
    ///Position in pixels
    position: Vec2,
    ///Size in pixels
    size: Vec2,
    ///Color (RGBA)
    color: Corners<Vec4>,
    ///Rounded corners
    rounded_corners: Option<RoundedCorners>,
  },
  /// Filled, colored circle
  Circle {
    ///Position in pixels
    position: Vec2,
    ///Radius in pixels
    radius: f32,
    ///Color (RGBA)
    color: Vec4,
  },
  /// Draw text using the specified font, size, color, and position
  Text {
    ///Position in pixels
    position: Vec2,
    ///Font size
    size: u16,
    ///Color (RGBA)
    color: Vec4,
    ///Text to draw
    text: Cow<'static, str>,
    ///Font handle to use
    font: FontHandle,
  },
}

/// List of draw commands
#[derive(Default)]
pub struct UiDrawCommandList {
  pub commands: Vec<UiDrawCommand>,
}

impl UiDrawCommandList {
  /// Add a draw command to the list
  pub fn add(&mut self, command: UiDrawCommand) {
    self.commands.push(command);
  }
}

// impl UiDrawCommands {
//   pub fn compare(&self, other: &Self) -> bool {
//     // if self.commands.len() != other.commands.len() { return false }
//     // self.commands.iter().zip(other.commands.iter()).all(|(a, b)| a == b)
//   }
// }

/// A vertex for UI rendering
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct UiVertex {
  pub position: Vec2,
  pub color: Vec4,
  pub uv: Vec2,
}

/// Represents a single draw call, should be handled by the render backend
#[derive(Default)]
pub struct UiDrawCall {
  pub vertices: Vec<UiVertex>,
  pub indices: Vec<u32>,
}

impl UiDrawCall {
  /// Tesselate the UI and build a complete draw plan from a list of draw commands
  pub(crate) fn build(draw_commands: &UiDrawCommandList, atlas: &mut TextureAtlasManager, text_renderer: &mut TextRenderer) -> Self {
    let mut draw_call = UiDrawCall::default();
    for command in &draw_commands.commands {
      match command {
        UiDrawCommand::Rectangle { position, size, color, rounded_corners } => {
          let vidx = draw_call.vertices.len() as u32;
          if let Some(corner) = rounded_corners.filter(|x| x.radius.max_f32() > 0.0) {
            //this code is stupid as fuck

            //Random vert in the center for no reason
            //lol
            draw_call.vertices.push(UiVertex {
              position: *position + *size * vec2(0.5, 0.5),
              color: (color.bottom_left + color.bottom_right + color.top_left + color.top_right) / 4.,
              uv: vec2(0., 0.),
            });

            //TODO: fix some corners tris being invisible (but it's already close enough lol)
            let rounded_corner_verts = corner.point_count.get() as u32;
            for i in 0..rounded_corner_verts {
              let cratio = i as f32 / rounded_corner_verts as f32;
              let angle = cratio * std::f32::consts::PI * 0.5;
              let x = angle.sin();
              let y = angle.cos();
              //Top-right corner
              draw_call.vertices.push(UiVertex {
                position: *position + vec2(x, 1. - y) * corner.radius.top_right + vec2(size.x - corner.radius.top_right, 0.),
                color: color.top_right,
                uv: vec2(0.0, 0.0),
              });
              //Bottom-right corner
              draw_call.vertices.push(UiVertex {
                position: *position + vec2(x - 1., y) * corner.radius.bottom_right + vec2(size.x, size.y - corner.radius.bottom_right),
                color: color.bottom_right,
                uv: vec2(0.0, 0.0),
              });
              //Bottom-left corner
              draw_call.vertices.push(UiVertex {
                position: *position + vec2(1. - x, y) * corner.radius.bottom_left + vec2(0., size.y - corner.radius.bottom_left),
                color: color.bottom_left,
                uv: vec2(0.0, 0.0),
              });
              //Top-left corner
              draw_call.vertices.push(UiVertex {
                position: *position + vec2(1. - x, 1. - y) * corner.radius.top_left,
                color: color.top_left,
                uv: vec2(0.0, 0.0),
              });
              // mental illness:
              if i > 0 {
                draw_call.indices.extend([
                  //Top-right corner
                  vidx,
                  vidx + 1 + (i - 1) * 4,
                  vidx + 1 + i * 4,
                  //Bottom-right corner
                  vidx,
                  vidx + 1 + (i - 1) * 4 + 1,
                  vidx + 1 + i * 4 + 1,
                  //Bottom-left corner
                  vidx,
                  vidx + 1 + (i - 1) * 4 + 2,
                  vidx + 1 + i * 4 + 2,
                  //Top-left corner
                  vidx,
                  vidx + 1 + (i - 1) * 4 + 3,
                  vidx + 1 + i * 4 + 3,
                ]);
              }
            }
            //Fill in the rest
            //mental illness 2:
            draw_call.indices.extend([
              //Top
              vidx,
              vidx + 4,
              vidx + 1,
              //Right?, i think
              vidx,
              vidx + 1 + (rounded_corner_verts - 1) * 4,
              vidx + 1 + (rounded_corner_verts - 1) * 4 + 1,
              //Left???
              vidx,
              vidx + 1 + (rounded_corner_verts - 1) * 4 + 2,
              vidx + 1 + (rounded_corner_verts - 1) * 4 + 3,
              //Bottom???
              vidx,
              vidx + 3,
              vidx + 2,
            ]);
          } else {
            draw_call.indices.extend([vidx, vidx + 1, vidx + 2, vidx, vidx + 2, vidx + 3]);
            draw_call.vertices.extend([
              UiVertex {
                position: *position,
                color: color.top_left,
                uv: vec2(0.0, 0.0),
              },
              UiVertex {
                position: *position + vec2(size.x, 0.0),
                color: color.top_right,
                uv: vec2(0.0, 0.0), // vec2(1.0, 0.0),
              },
              UiVertex {
                position: *position + *size,
                color: color.bottom_right,
                uv: vec2(0.0, 0.0), // vec2(1.0, 1.0),
              },
              UiVertex {
                position: *position + vec2(0.0, size.y),
                color: color.bottom_left,
                uv: vec2(0.0, 0.0), // vec2(0.0, 1.0),
              },
            ]);
          }
        },
        UiDrawCommand::Circle { .. } => {
          todo!("circle draw command not implemented yet")
        },
        UiDrawCommand::Text { position, size, color, text, font: font_handle } => {
          if text.is_empty() {
            continue
          }

          //XXX: should we be doing this every time?
          let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
          layout.append(
            &[text_renderer.internal_font(*font_handle)],
            &TextStyle::new(text, *size as f32, 0)
          );
          let glyphs = layout.glyphs();

          //let mut rpos_x = 0.;
          for layout_glyph in glyphs {
            if !layout_glyph.char_data.rasterize() {
              continue
            }
            let atlas_size = atlas.meta().size.as_vec2();
            let vidx = draw_call.vertices.len() as u32;
            let glyph = text_renderer.glyph(atlas, *font_handle, layout_glyph.parent, layout_glyph.key.px as u8);
            let uv = atlas.get_uv(glyph.texture);
            //rpos_x += glyph.metrics.advance_width;//glyph.metrics.advance_width;
            draw_call.indices.extend([vidx, vidx + 1, vidx + 2, vidx, vidx + 2, vidx + 3]);
            draw_call.vertices.extend([
              UiVertex {
                position: *position + vec2(layout_glyph.x, layout_glyph.y),
                color: *color,
                uv: uv.top_left,
              },
              UiVertex {
                position: *position + vec2(layout_glyph.x + glyph.metrics.width as f32, layout_glyph.y),
                color: *color,
                uv: uv.top_right,
              },
              UiVertex {
                position: *position + vec2(layout_glyph.x + glyph.metrics.width as f32, layout_glyph.y + glyph.metrics.height as f32),
                color: *color,
                uv: uv.bottom_right,
              },
              UiVertex {
                position: *position + vec2(layout_glyph.x, layout_glyph.y + glyph.metrics.height as f32),
                color: *color,
                uv: uv.bottom_left,
              },
            ]);
            #[cfg(all(
              feature = "pixel_perfect_text",
              not(feature = "pixel_perfect")
            ))] {
              for vtx in &mut draw_call.vertices[(vidx as usize)..] {
                vtx.position = vtx.position.round()
              }
            }
          }
        }
      }
    }
    #[cfg(feature = "pixel_perfect")]
    draw_call.vertices.iter_mut().for_each(|v| {
      v.position = v.position.round()
    });
    draw_call
  }
}
