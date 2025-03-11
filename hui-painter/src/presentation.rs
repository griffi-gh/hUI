use crate::{
  PainterInstance,
  paint::{
    buffer::PaintBuffer,
    command::PaintRoot,
  },
};

pub struct Presentatation {
  current_buffer: PaintBuffer,
  cur_hash: Option<u64>,
  prev_hash: Option<u64>,
  version_counter: u64,
}

impl Presentatation {
  pub fn new() -> Self {
    Self {
      current_buffer: PaintBuffer::new(),
      cur_hash: None,
      prev_hash: None,
      version_counter: 0,
    }
  }

  /// If the paint command has changed since the last draw call, draw it and return true.\
  /// Otherwise, returns false.
  pub fn draw(&mut self, painter: &mut PainterInstance, cmd: &impl PaintRoot) -> bool {
    self.prev_hash = self.cur_hash;
    self.cur_hash = Some(cmd.cache_hash());

    if self.prev_hash == self.cur_hash {
      return false;
    }

    self.current_buffer.clear();
    cmd.paint_root(painter, &mut self.current_buffer);

    self.version_counter = self.version_counter.wrapping_add(1);

    true
  }

  /// Get the current paint buffer
  pub fn buffer(&self) -> &PaintBuffer {
    &self.current_buffer
  }

  /// Get the complete backend data for the current presentation
  ///
  /// It contains the current paint buffer and the hash of the presentation\
  /// Unlike the `TextureAtlasBackendData`, the version is non-incremental
  pub fn backend_data(&self) -> PresentatationBackendData {
    PresentatationBackendData {
      buffer: &self.current_buffer,
      version: self.version_counter,
      hash: self.cur_hash.unwrap_or(0),
    }
  }
}

impl Default for Presentatation {
  fn default() -> Self {
    Self::new()
  }
}


/// Backend data for the Presentation
#[derive(Clone, Copy)]
pub struct PresentatationBackendData<'a> {
  /// The current paint buffer
  pub buffer: &'a PaintBuffer,

  /// The version of the presentation
  ///
  /// This is incremented every time the buffer hash changes
  pub version: u64,

  /// Unique hash of current paint buffer commands
  pub hash: u64,
}

