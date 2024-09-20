use crate::{paint::{
  buffer::PaintBuffer,
  command::PaintCommand,
}, Painter};

//TODO: use generics instead

pub struct PaintTransform {
  pub transform: glam::Affine2,
  pub children: Vec<Box<dyn PaintCommand>>,
}

impl PaintCommand for PaintTransform {
  fn paint(&self, ctx: &mut Painter, into: &mut PaintBuffer) {
    // remember the starting index
    let starting_index = into.vertices.len();

    // paint children nodes
    for child in &self.children {
      child.paint(ctx, into);
    }

    let mut min_point = glam::Vec2::splat(f32::MAX);
    let mut max_point = glam::Vec2::splat(f32::MIN);
    for vtx in &into.vertices[starting_index..] {
      min_point = min_point.min(vtx.position);
      max_point = max_point.max(vtx.position);
    }

    // trans the children in-place
    for vtx in &mut into.vertices[starting_index..] {
      //HACK: to match the old behavior:
      //(shift the origin to the center before transforming)
      let offset = (max_point + min_point) / 2.0;
      vtx.position -= offset;
      vtx.position = self.transform.transform_point2(vtx.position);
      vtx.position += offset;
    }
  }
}
