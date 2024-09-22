use glam::Vec2;
use crate::{paint::buffer::PaintBuffer, Painter};

// mod root;
// pub use root::RootCommand;

mod transform;
pub use transform::PaintTransform;

mod rectangle;
pub use rectangle::PaintRectangle;

mod text;
pub use text::PaintText;

pub trait PaintCommand {
  /// Called before actual paint command is executed\
  /// Opportunity to pre-cache bitmaps, etc.
  ///
  /// Make sure to propagate this call to children!
  #[allow(unused_variables)]
  fn pre_paint(&self, ctx: &mut Painter) {}

  /// Paint the command into the buffer
  ///
  /// Do not allocate new textures or cache glyphs here, use `pre_paint` instead!\
  /// (Doing this WILL lead to atlas corruption flicker for a single frame if it's forced to resize!)
  fn paint(&self, ctx: &mut Painter, into: &mut PaintBuffer);
}

pub trait Measurable: PaintCommand {
  fn size(&self, ctx: &Painter) -> Vec2;
}
