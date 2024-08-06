use crate::paint::{
  buffer::PaintBuffer,
  command::PaintCommand,
};

pub struct PaintText {
  //TODO: PaintText command
}

impl PaintCommand for PaintText {
  fn paint(&self, into: &mut PaintBuffer) {
    todo!()
  }
}
