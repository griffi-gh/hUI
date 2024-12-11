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
}