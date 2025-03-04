use glam::{Vec2, Vec4};

pub struct Vertex {
  pub position: Vec2, //Vec3,
  pub uv: Vec2,
  pub color: Vec4,
}

pub struct PaintBuffer {
  pub vertices: Vec<Vertex>,
  pub indices: Vec<u32>,
}

impl PaintBuffer {
  pub fn new() -> Self {
    Self {
      vertices: Vec::new(),
      indices: Vec::new(),
    }
  }
}

impl Default for PaintBuffer {
  fn default() -> Self {
    Self::new()
  }
}
