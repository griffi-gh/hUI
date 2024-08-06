use crate::paint::buffer::PaintBuffer;

mod transform;
pub use transform::PaintTransform;

mod rectangle;
pub use rectangle::PaintRectangle;

mod text;
pub use text::PaintText;

pub trait PaintCommand {
  fn paint(&self, into: &mut PaintBuffer);
}
