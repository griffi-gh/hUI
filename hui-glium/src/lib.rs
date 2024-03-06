use std::rc::Rc;
use glam::Vec2;
use glium::{
  Surface, DrawParameters, Blend,
  Program, VertexBuffer, IndexBuffer,
  backend::{Facade, Context},
  texture::{SrgbTexture2d, RawImage2d},
  index::PrimitiveType,
  implement_vertex,
  uniform, uniforms::{Sampler, SamplerBehavior, SamplerWrapFunction},
};
use hui::{
  draw::{TextureAtlasMeta, UiDrawCall, UiVertex}, UiInstance
};

const VERTEX_SHADER: &str = include_str!("../shaders/vertex.vert");
const FRAGMENT_SHADER: &str = include_str!("../shaders/fragment.frag");

#[derive(Clone, Copy)]
#[repr(C)]
struct Vertex {
  position: [f32; 2],
  color: [f32; 4],
  uv: [f32; 2],
}

impl From<UiVertex> for Vertex {
  fn from(v: UiVertex) -> Self {
    Self {
      position: v.position.to_array(),
      color: v.color.to_array(),
      uv: v.uv.to_array(),
    }
  }
}

implement_vertex!(Vertex, position, color, uv);

struct BufferPair {
  pub vertex_buffer: glium::VertexBuffer<Vertex>,
  pub index_buffer: glium::IndexBuffer<u32>,
  pub vertex_count: usize,
  pub index_count: usize,
}

impl BufferPair {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::debug!("init ui buffers (empty)...");
    Self {
      vertex_buffer: VertexBuffer::empty_dynamic(facade, 1024).unwrap(),
      index_buffer: IndexBuffer::empty_dynamic(facade, PrimitiveType::TrianglesList, 1024).unwrap(),
      vertex_count: 0,
      index_count: 0,
    }
  }

  pub fn new_with_data<F: Facade>(facade: &F, vtx: &[Vertex], idx: &[u32]) -> Self {
    log::debug!("init ui buffers (data)...");
    Self {
      vertex_buffer: VertexBuffer::dynamic(facade, vtx).unwrap(),
      index_buffer: IndexBuffer::dynamic(facade, PrimitiveType::TrianglesList, idx).unwrap(),
      vertex_count: vtx.len(),
      index_count: idx.len(),
    }
  }

  pub fn ensure_buffer_size(&mut self, need_vtx: usize, need_idx: usize) {
    let current_vtx_size = self.vertex_buffer.get_size() / std::mem::size_of::<Vertex>();
    let current_idx_size = self.index_buffer.get_size() / std::mem::size_of::<u32>();
    //log::debug!("current vtx size: {}, current idx size: {}", current_vtx_size, current_idx_size);
    if current_vtx_size >= need_vtx && current_idx_size >= need_idx {
      return
    }
    let new_vtx_size = (need_vtx + 1).next_power_of_two();
    let new_idx_size = (need_idx + 1).next_power_of_two();
    log::debug!("resizing buffers: vtx {} -> {}, idx {} -> {}", current_vtx_size, new_vtx_size, current_idx_size, new_idx_size);
    if current_vtx_size != new_vtx_size {
      self.vertex_buffer = VertexBuffer::empty_dynamic(
        self.vertex_buffer.get_context(),
        new_vtx_size
      ).unwrap();
    }
    if current_idx_size != new_idx_size {
      self.index_buffer = IndexBuffer::empty_dynamic(
        self.index_buffer.get_context(),
        PrimitiveType::TrianglesList,
        new_idx_size
      ).unwrap();
    }
  }

  pub fn write_data(&mut self, vtx: &[Vertex], idx: &[u32]) {
    //log::trace!("uploading {} vertices and {} indices", vtx.len(), idx.len());

    self.vertex_count = vtx.len();
    self.index_count = idx.len();

    if self.vertex_count == 0 || self.index_count == 0 {
      self.vertex_buffer.invalidate();
      self.index_buffer.invalidate();
      return
    }

    self.ensure_buffer_size(self.vertex_count, self.index_count);

    self.vertex_buffer.slice_mut(0..self.vertex_count).unwrap().write(vtx);
    self.index_buffer.slice_mut(0..self.index_count).unwrap().write(idx);
  }

  pub fn is_empty(&self) -> bool {
    self.vertex_count == 0 || self.index_count == 0
  }
}

pub struct GliumUiRenderer {
  context: Rc<Context>,
  program: glium::Program,
  ui_texture: Option<SrgbTexture2d>,
  buffer_pair: Option<BufferPair>,
}

impl GliumUiRenderer {
  pub fn new<F: Facade>(facade: &F) -> Self {
    log::info!("initializing hui-glium");
    Self {
      program: Program::from_source(facade, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap(),
      context: Rc::clone(facade.get_context()),
      ui_texture: None,
      buffer_pair: None,
    }
  }

  fn update_buffers(&mut self, call: &UiDrawCall) {
    log::trace!("updating ui buffers (tris: {})", call.indices.len() / 3);
    let data_vtx = &call.vertices.iter().copied().map(Vertex::from).collect::<Vec<_>>()[..];
    let data_idx = &call.indices[..];
    if let Some(buffer) = &mut self.buffer_pair {
      buffer.write_data(data_vtx, data_idx);
    } else if !call.indices.is_empty() {
      self.buffer_pair = Some(BufferPair::new_with_data(&self.context, data_vtx, data_idx));
    }
  }

  fn update_texture_atlas(&mut self, atlas: &TextureAtlasMeta) {
    log::trace!("updating ui atlas texture");
    self.ui_texture = Some(SrgbTexture2d::new(
      &self.context,
      RawImage2d::from_raw_rgba(
        atlas.data.to_owned(),
        (atlas.size.x, atlas.size.y)
      )
    ).unwrap());
  }

  pub fn update(&mut self, instance: &UiInstance) {
    if self.ui_texture.is_none() || instance.atlas().modified {
      self.update_texture_atlas(&instance.atlas());
    }
    if self.buffer_pair.is_none() || instance.draw_call().0 {
      self.update_buffers(instance.draw_call().1);
    }
  }

  pub fn draw(&self, frame: &mut glium::Frame, resolution: Vec2) {
    let params = DrawParameters {
      blend: Blend::alpha_blending(),
      ..Default::default()
    };

    if let Some(buffer) = &self.buffer_pair {
      if buffer.is_empty() {
        return
      }

      let vtx_buffer = buffer.vertex_buffer.slice(0..buffer.vertex_count).unwrap();
      let idx_buffer = buffer.index_buffer.slice(0..buffer.index_count).unwrap();

      frame.draw(
        vtx_buffer,
        idx_buffer,
        &self.program,
        &uniform! {
          resolution: resolution.to_array(),
          tex: Sampler(self.ui_texture.as_ref().unwrap(), SamplerBehavior {
            wrap_function: (SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp, SamplerWrapFunction::Clamp),
            ..Default::default()
          }),
        },
        &params,
      ).unwrap();
    }
  }
}
