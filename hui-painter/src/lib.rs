//TODO painter rewrite

mod rect;
pub use rect::PaintRectParams;

pub struct PaintTransformParams {
  transform: glam::Affine2,
}

pub enum PaintCommand {
  Rect(PaintRectParams),
  Transform(PaintTransformParams, Box<PaintCommand>),
}

pub struct Painter {

}
