use crate::paint::{
  buffer::PaintBuffer,
  command::PaintCommand,
};

//TODO: use generics instead

pub struct PaintTransform {
  pub transform: glam::Affine2,
  pub children: Vec<Box<dyn PaintCommand>>,
}

impl PaintCommand for PaintTransform {
  fn paint(&self, into: &mut PaintBuffer) {
    // remember the starting index
    let starting_index = into.vertices.len();

    // paint children nodes
    for child in &self.children {
      child.paint(into);
    }

    // trans the children in-place
    for vtx in &mut into.vertices[starting_index..] {
      //TODO fix for rotation around the center of the object
      vtx.position = self.transform.transform_point2(vtx.position);
    }
  }
}
