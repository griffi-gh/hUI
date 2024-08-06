use glam::{UVec2, uvec2, ivec2};
use rect_packer::DensePacker;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

//TODO support rotation
const ALLOW_ROTATION: bool = false;

const RGBA_BYTES_PER_PIXEL: usize = 4;

fn assert_size(size: UVec2) {
  assert!(
    size.x > 0 &&
    size.y > 0,
    "size must be greater than 0"
  );
  assert!(
    size.x <= i32::MAX as u32 &&
    size.y <= i32::MAX as u32,
    "size must be less than i32::MAX"
  );
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SourceTextureFormat {
  /// RGBA, 8-bit per channel\
  /// (Default and preferred format)
  #[default]
  RGBA8,

  /// RGB, 8-bit per channel
  /// (Alpha channel is assumed to be 255)
  RGB8,

  /// Alpha only, 8-bit per channel
  /// (All other channels are assumed to be 255 (white))
  A8,

  //TODO ARGB, BGRA, etc.
}

impl SourceTextureFormat {
  pub const fn bytes_per_pixel(&self) -> usize {
    match self {
      SourceTextureFormat::RGBA8 => 4,
      SourceTextureFormat::RGB8 => 3,
      SourceTextureFormat::A8 => 1,
    }
  }
}

pub type TextureId = u32;

#[derive(Clone, Copy)]
pub struct TextureHandle {
  pub(crate) id: TextureId,
  pub(crate) size: UVec2,
}

struct TextureAllocation {
  handle: TextureHandle,
  offset: UVec2,
  size: UVec2,
}

pub struct TextureAtlas {
  size: UVec2,
  data: Vec<u8>,
  packer: DensePacker,
  next_id: TextureId,
  allocations: HashMap<TextureId, TextureAllocation, BuildNoHashHasher<TextureId>>,
}

impl TextureAtlas {
  pub fn new(size: UVec2) -> Self {
    assert_size(size);
    let data_bytes = (size.x * size.y) as usize * RGBA_BYTES_PER_PIXEL;
    Self {
      size,
      data: vec![0; data_bytes],
      packer: DensePacker::new(
        size.x as i32,
        size.y as i32,
      ),
      next_id: 0,
      allocations: HashMap::default(),
    }
  }

  fn next_handle(&mut self, size: UVec2) -> TextureHandle {
    let handle = TextureHandle {
      id: self.next_id,
      size,
    };
    self.next_id += 1;
    handle
  }


  /// Allocate a texture in the atlas, returning a handle to it.\
  /// The data present in the texture is undefined, and may include garbage data.
  ///
  /// # Panics
  /// - If any of the dimensions of the texture are zero or exceed `i32::MAX`.
  pub fn allocate(&mut self, size: UVec2) -> TextureHandle {
    assert_size(size);

    // Pack the texture
    let pack = self.packer.pack(
      size.x as i32,
      size.y as i32,
      ALLOW_ROTATION
    );

    //TODO: handle pack failure by either resizing the atlas or returning an error
    let pack = pack.unwrap();
    let offset = ivec2(pack.x, pack.y).as_uvec2();

    // Allocate the texture
    let handle = self.next_handle(size);
    let allocation = TextureAllocation { handle, offset, size };
    self.allocations.insert_unique_unchecked(handle.id, allocation);

    handle
  }

  /// Allocate a texture in the atlas, returning a handle to it.\
  /// The texture is initialized with the provided data.
  ///
  /// The source data must be in the format specified by the `format` parameter.\
  /// (Please note that the internal format of the texture is always RGBA8, regardless of the source format.)
  ///
  /// # Panics
  /// - If any of the dimensions of the texture are zero or exceed `i32::MAX`.
  /// - The length of the data array is zero or not a multiple of the stride (stride = width * bytes per pixel).
  pub fn allocate_with_data(&mut self, format: SourceTextureFormat, data: &[u8], width: usize) -> TextureHandle {
    assert!(
      !data.is_empty(),
      "texture data must not be empty"
    );

    // Calculate the stride of the texture
    let bytes_per_pixel = format.bytes_per_pixel();
    let stride = bytes_per_pixel * width;
    assert_eq!(
      data.len() % stride, 0,
      "texture data must be a multiple of the stride",
    );

    // Calculate the size of the texture
    let size = uvec2(
      width as u32,
      (data.len() / stride) as u32,
    );
    assert_size(size);

    // Allocate the texture
    let handle = self.allocate(size);
    let allocation = self.allocations.get(&handle.id).unwrap();

    for y in 0..size.y {
      for x in 0..size.x {
        let src_idx = (y * size.x + x) as usize * bytes_per_pixel;
        let dst_idx: usize = (
          (allocation.offset.y + y) * size.x +
          (allocation.offset.x + x)
        ) as usize * RGBA_BYTES_PER_PIXEL;

        let src = &data[src_idx..src_idx + bytes_per_pixel];
        let dst = &mut self.data[dst_idx..dst_idx + RGBA_BYTES_PER_PIXEL];

        match format {
          SourceTextureFormat::RGBA8 => {
            dst.copy_from_slice(src);
          }
          SourceTextureFormat::RGB8 => {
            dst[..3].copy_from_slice(src);
            dst[3] = 255;
          }
          SourceTextureFormat::A8 => {
            dst[0] = src[0];
            dst[1] = src[0];
            dst[2] = src[0];
            dst[3] = 255;
          }
        }
      }
    }

    handle
  }
}
