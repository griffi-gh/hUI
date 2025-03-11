use hui_shared::rect::Rect;
use crate::{paint::buffer::PaintBuffer, PainterInstance};

// mod root;
// pub use root::RootCommand;

mod list;
pub use list::PaintList;

mod transform;
pub use transform::PaintTransform;

mod rectangle;
pub use rectangle::PaintRectangle;

pub mod text;

pub trait PaintCommand {
  /// Called before actual paint command is executed\
  /// Opportunity to pre-cache bitmaps, etc.
  ///
  /// Make sure to propagate this call to children!
  #[allow(unused_variables)]
  fn pre_paint(&self, ctx: &mut PainterInstance) {}

  /// Paint the command into the buffer
  ///
  /// Do not allocate new textures or cache glyphs here, use `pre_paint` instead!\
  /// (Doing this WILL lead to atlas corruption flicker for a single frame if it's forced to resize!)
  fn paint(&self, ctx: &mut PainterInstance, into: &mut PaintBuffer);

  /// Hash of the parameters that affect command's appearance
  ///
  /// Must be unique for each possilbe combination of parameters
  fn cache_hash(&self) -> u64;

  fn bounds(&self, ctx: &PainterInstance) -> Rect;
}

// TODO move paint_root to PaintCommand instead of separate trait?
pub trait PaintRoot: PaintCommand {
  /// Paint the root command, calling `pre_paint` before painting
  ///
  /// This is a convenience method for painting the root command
  /// Do not use this inside the `paint` method of a command!
  fn paint_root(&self, ctx: &mut PainterInstance, into: &mut PaintBuffer) {
    self.pre_paint(ctx);
    self.paint(ctx, into);
  }
}

impl<T: PaintCommand> PaintRoot for T {}
