use std::{hash::Hasher, ops::RangeFull};
use hui_shared::rect::Rect;
use crate::PainterInstance;
use super::PaintCommand;

pub struct PaintList {
  pub commands: Vec<Box<dyn PaintCommand>>,
}

impl PaintList {
  pub fn new(commands: Vec<Box<dyn PaintCommand>>) -> Self {
    Self {
      commands
    }
  }

  pub fn new_empty() -> Self {
    Self {
      commands: Vec::new(),
    }
  }

  pub fn add(&mut self, command: impl PaintCommand + 'static) {
    self.commands.push(Box::new(command));
  }
}

impl Default for PaintList {
  fn default() -> Self {
    Self::new_empty()
  }
}

impl PaintCommand for PaintList {
  fn pre_paint(&self, ctx: &mut PainterInstance) {
    for command in &self.commands {
      command.pre_paint(ctx);
    }
  }

  fn paint(&self, ctx: &mut crate::PainterInstance, into: &mut crate::paint::buffer::PaintBuffer) {
    for command in &self.commands {
      command.paint(ctx, into);
    }
  }

  fn cache_hash(&self) -> u64 {
    let mut hasher = rustc_hash::FxHasher::default();
    for command in self.commands.iter() {
      hasher.write_u64(command.cache_hash());
    }
    hasher.finish()
  }

  fn bounds(&self, ctx: &PainterInstance) -> Rect {
    self.list_bounds_partial(ctx, ..)
  }
}

impl PaintList {
  /// Get the bounds of a range of commands stored in this list
  pub fn list_bounds_partial(&self, ctx: &PainterInstance, range: RangeFull) -> Rect {
    let selector = &self.commands[range];
    if selector.is_empty() {
      return Rect::ZERO;
    }
    let mut position = glam::Vec2::splat(f32::MAX);
    let mut size = glam::Vec2::splat(f32::MIN);
    for command in selector {
      let bounds = command.bounds(ctx);
      position = position.min(bounds.position);
      size = size.max(bounds.size);
    }
    Rect {
      position,
      size,
    }
  }
}