use crate::{paint::buffer::PaintBuffer, Painter};

mod transform;
pub use transform::PaintTransform;

mod rectangle;
pub use rectangle::PaintRectangle;

mod text;
pub use text::PaintText;

pub trait PaintCommand {
  fn paint(&self, ctx: &mut Painter, into: &mut PaintBuffer);
}
