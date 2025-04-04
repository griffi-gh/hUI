use glam::{vec2, Vec2};
use hui_painter::{
  backend::BackendData,
  paint::buffer::Vertex,
  presentation::PresentatationBackendData,
  texture::TextureAtlasBackendData
};

const DEFAULT_BUFFER_SIZE: u64 = 1024;
const DEFAULT_TEXTURE_SIZE: u32 = 512;
const SHADER_MODULE: &str = include_str!("../shaders/ui.wgsl");

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct WgpuVertex {
  position: [f32; 2],
  uv: [f32; 2],
  color: [f32; 4],
}
unsafe impl bytemuck::Pod for WgpuVertex {}
unsafe impl bytemuck::Zeroable for WgpuVertex {}

impl WgpuVertex {
  pub const LAYOUT: wgpu::VertexBufferLayout<'static> = wgpu::VertexBufferLayout {
    array_stride: std::mem::size_of::<WgpuVertex>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![
      0 => Float32x2,
      1 => Float32x2,
      2 => Float32x4,
    ],
  };
}

impl From<Vertex> for WgpuVertex {
  fn from(v: Vertex) -> Self {
    Self {
      position: v.position.to_array(),
      uv: v.uv.to_array(),
      color: v.color.to_array(),
    }
  }
}

pub struct WgpuUiRenderer {
  // pub modified: bool,
  pub last_buf_hash: u64,
  pub last_img_version: u64,
  pub vertex_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub vertex_count: usize,
  pub index_count: usize,
  pub bind_group_layout: wgpu::BindGroupLayout,
  pub bind_group: wgpu::BindGroup,
  pub pipeline: wgpu::RenderPipeline,
  pub texture: wgpu::Texture,
  pub texture_view: wgpu::TextureView,
  pub texture_sampler: wgpu::Sampler,
}

impl WgpuUiRenderer {
  pub fn new(
    device: &wgpu::Device,
    surface_format: wgpu::TextureFormat,
  ) -> Self {
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("ui_vertex_buffer"),
      size: std::mem::size_of::<WgpuVertex>() as u64 * DEFAULT_BUFFER_SIZE,
      usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
      label: Some("hui_index_buffer"),
      size: std::mem::size_of::<u32>() as u64 * DEFAULT_BUFFER_SIZE,
      usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
      mapped_at_creation: false,
    });

    let texture = device.create_texture(&wgpu::TextureDescriptor {
      label: Some("ui_texture"),
      size: wgpu::Extent3d {
        width: DEFAULT_TEXTURE_SIZE,
        height: DEFAULT_TEXTURE_SIZE,
        depth_or_array_layers: 1,
      },
      mip_level_count: 1,
      sample_count: 1,
      dimension: wgpu::TextureDimension::D2,
      format: wgpu::TextureFormat::Rgba8UnormSrgb,
      usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    });

    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
      label: Some("ui_bind_group_layout"),
      entries: &[
        wgpu::BindGroupLayoutEntry {
          binding: 0,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: false },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
          },
          count: None,
        },
        wgpu::BindGroupLayoutEntry {
          binding: 1,
          visibility: wgpu::ShaderStages::FRAGMENT,
          ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
          count: None,
        },
      ],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor {
      label: Some("ui_texture_view"),
      ..Default::default()
    });

    let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
      label: Some("ui_texture_sampler"),
      address_mode_u: wgpu::AddressMode::ClampToEdge,
      address_mode_v: wgpu::AddressMode::ClampToEdge,
      address_mode_w: wgpu::AddressMode::ClampToEdge,
      mag_filter: wgpu::FilterMode::Nearest,
      min_filter: wgpu::FilterMode::Nearest,
      mipmap_filter: wgpu::FilterMode::Nearest,
      ..Default::default()
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
      label: Some("ui_bind_group"),
      layout: &bind_group_layout,
      entries: &[
        wgpu::BindGroupEntry {
          binding: 0,
          resource: wgpu::BindingResource::TextureView(&texture_view),
        },
        wgpu::BindGroupEntry {
          binding: 1,
          resource: wgpu::BindingResource::Sampler(&texture_sampler),
        },
      ],
    });

    let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
      label: Some("ui_vertex_shader"),
      source: wgpu::ShaderSource::Wgsl(SHADER_MODULE.into()),
    });

    let pipeline = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
      label: Some("ui_pipeline_layout"),
      bind_group_layouts: &[&bind_group_layout],
      push_constant_ranges: &[],
    });

    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
      label: Some("ui_pipeline"),
      layout: Some(&pipeline),
      vertex: wgpu::VertexState {
        module: &shader_module,
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        entry_point: Some("vs_main"),
        buffers: &[WgpuVertex::LAYOUT],
      },
      fragment: Some(wgpu::FragmentState {
        module: &shader_module,
        compilation_options: wgpu::PipelineCompilationOptions::default(),
        entry_point: Some("fs_main"),
        targets: &[Some(wgpu::ColorTargetState {
          format: surface_format,
          blend: Some(wgpu::BlendState::ALPHA_BLENDING),
          write_mask: wgpu::ColorWrites::COLOR,
        })],
      }),
      primitive: wgpu::PrimitiveState {
        topology: wgpu::PrimitiveTopology::TriangleList,
        strip_index_format: None,
        front_face: wgpu::FrontFace::Ccw,
        cull_mode: None,
        polygon_mode: wgpu::PolygonMode::Fill,
        conservative: false,
        unclipped_depth: false,
      },
      depth_stencil: None,
      multisample: wgpu::MultisampleState::default(),
      multiview: None,
      cache: None,
    });

    Self {
      last_buf_hash: 0,
      last_img_version: 0,
      vertex_buffer,
      index_buffer,
      vertex_count: 0,
      index_count: 0,
      bind_group_layout,
      bind_group,
      texture,
      texture_view,
      texture_sampler,
      pipeline,
    }
  }

  fn update_buffers(&mut self, present_data: &PresentatationBackendData, queue: &wgpu::Queue, device: &wgpu::Device, resolution: Vec2) {
    let data_vtx = present_data.buffer.vertices.iter()
      .copied()
      .map(|x| {
        let mut v = x;
        v.position = vec2(1., -1.) * ((v.position / resolution) * 2.0 - 1.0);
        v
      })
      .map(WgpuVertex::from)
      .collect::<Vec<_>>();
    let data_idx = &present_data.buffer.indices[..];

    let data_vtx_view = bytemuck::cast_slice(&data_vtx);
    let data_idx_view = bytemuck::cast_slice(data_idx);

    self.vertex_count = present_data.buffer.vertices.len();
    self.index_count = present_data.buffer.indices.len();

    if data_vtx.is_empty() || data_idx.is_empty() {
      return
    }

    if data_vtx_view.len() as u64 > self.vertex_buffer.size() {
      self.vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("ui_vertex_buffer"),
        size: (data_vtx_view.len() as u64).next_power_of_two(),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
      });
    }

    if data_idx_view.len() as u64 > self.index_buffer.size() {
      self.index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("ui_index_buffer"),
        size: (data_idx_view.len() as u64).next_power_of_two(),
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
      });
    }

    queue.write_buffer(&self.vertex_buffer, 0, data_vtx_view);
    queue.write_buffer(&self.index_buffer, 0, data_idx_view);

    self.last_buf_hash = present_data.hash;
  }

  fn update_texture(&mut self, atlas: &TextureAtlasBackendData, queue: &wgpu::Queue, device: &wgpu::Device) {
    //TODO URGENCY:HIGH resize texture if needed
    if atlas.data.len() as u32 > (self.texture.size().width * self.texture.size().height * 4) {
      self.texture.destroy();
      // unimplemented!("texture resize not implemented");
      self.texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("ui_texture"),
        size: wgpu::Extent3d {
          width: DEFAULT_TEXTURE_SIZE,
          height: DEFAULT_TEXTURE_SIZE,
          depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
      });
    }
    queue.write_texture(
      wgpu::TexelCopyTextureInfo {
        texture: &self.texture,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
      },
      atlas.data,
      wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(atlas.size.x * 4),
        rows_per_image: Some(atlas.size.y),
      },
      wgpu::Extent3d {
        width: atlas.size.x,
        height: atlas.size.y,
        depth_or_array_layers: 1,
      }
    );

    self.last_img_version = atlas.version;
  }

  pub fn update(
    &mut self,
    data: &BackendData,
    queue: &wgpu::Queue,
    device: &wgpu::Device,
    resolution: Vec2,
  ) {
    if data.presentation.hash != self.last_buf_hash {
      self.update_buffers(&data.presentation, queue, device, resolution);
    }
    if data.atlas.version != self.last_img_version {
      self.update_texture(&data.atlas, queue, device);
    }
  }

  pub fn draw(
    &self,
    encoder: &mut wgpu::CommandEncoder,
    surface_view: &wgpu::TextureView,
  ) {
    if self.vertex_count == 0 || self.index_count == 0 {
      return
    }

    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
      label: Some("ui_render_pass"),
      color_attachments: &[Some(wgpu::RenderPassColorAttachment {
        view: surface_view,
        resolve_target: None,
        ops: wgpu::Operations {
          load: wgpu::LoadOp::Load,
          store: wgpu::StoreOp::Store,
        },
      })],
      ..Default::default()
    });

    let vtx_size = self.vertex_count as u64 * std::mem::size_of::<WgpuVertex>() as u64;
    let idx_size = self.index_count as u64 * std::mem::size_of::<u32>() as u64;

    rpass.set_pipeline(&self.pipeline);
    rpass.set_bind_group(0, &self.bind_group, &[]);
    rpass.set_vertex_buffer(0, self.vertex_buffer.slice(0..vtx_size));
    rpass.set_index_buffer(self.index_buffer.slice(..idx_size), wgpu::IndexFormat::Uint32);
    rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
  }
}
