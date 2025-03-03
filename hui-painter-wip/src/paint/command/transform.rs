use std::hash::Hasher;

use crate::{
  PainterInstance,
  paint::{
    buffer::PaintBuffer,
    command::PaintCommand,
  },
};

pub struct PaintTransform<T: PaintCommand + 'static> {
  pub transform: glam::Affine2,
  pub child: T,
}

impl<T: PaintCommand + 'static> PaintCommand for PaintTransform<T> {
  fn pre_paint(&self, ctx: &mut PainterInstance) {
    self.child.pre_paint(ctx);
  }

  fn paint(&self, ctx: &mut PainterInstance, into: &mut PaintBuffer) {
    // remember the starting index
    let starting_index = into.vertices.len();

    // paint children node
    self.child.paint(ctx, into);

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

  fn cache_hash(&self) -> u64 {
    let mut hasher = rustc_hash::FxHasher::default();
    hasher.write_u64(self.child.cache_hash());
    self.transform.to_cols_array().iter().for_each(|v| {
      hasher.write_u32(v.to_bits())
    });
    hasher.finish()
  }

  fn size(&self, ctx: &PainterInstance) -> glam::Vec2 {
    // TODO take transform into account
    self.child.size(ctx)
  }
}
