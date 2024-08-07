use glam::{UVec2, uvec2, ivec2};
use rect_packer::DensePacker;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

//TODO support rotation
const DEFAULT_ATLAS_SIZE: UVec2 = uvec2(128, 128);
// const ALLOW_ROTATION: bool = false;

// Destination format is always RGBA
const RGBA_BYTES_PER_PIXEL: usize = 4;

/// Assert that the passed texture size is valid, panicking if it's not.
///
/// - The size must be greater than 0.
/// - The size must be less than `i32::MAX`.
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

/// The format of the source texture data to use when updating a texture in the atlas.
#[derive(Clone, Copy, Debug, Default)]
pub enum SourceTextureFormat {
  /// RGBA, 8-bit per channel
  #[default]
  RGBA8,

  //TODO native-endian RGBA32 format

  /// ARGB, 8-bit per channel
  ARGB8,

  /// BGRA, 8-bit per channel
  BGRA8,

  /// ABGR, 8-bit per channel
  ABGR8,

  /// RGB, 8-bit per channel (Alpha = 255)
  RGB8,

  /// BGR, 8-bit per channel (Alpha = 255)
  BGR8,

  /// Alpha only, 8-bit per channel (RGB = #ffffff)
  A8,
}

impl SourceTextureFormat {
  pub const fn bytes_per_pixel(&self) -> usize {
    match self {
      SourceTextureFormat::RGBA8 |
      SourceTextureFormat::ARGB8 |
      SourceTextureFormat::BGRA8 |
      SourceTextureFormat::ABGR8 => 4,
      SourceTextureFormat::RGB8 |
      SourceTextureFormat::BGR8 => 3,
      SourceTextureFormat::A8 => 1,
    }
  }
}

type TextureId = u32;

/// A handle to a texture in the texture atlas.
///
/// Can be cheaply copied and passed around.\
/// The handle is only valid for the texture atlas it was created from.
#[derive(Clone, Copy)]
pub struct TextureHandle {
  pub(crate) id: TextureId,
  pub(crate) size: UVec2,
}

impl TextureHandle {
  pub fn size(&self) -> UVec2 {
    self.size
  }
}

/// Represents an area allocated to a specific texture handle in the texture atlas.
struct TextureAllocation {
  /// Corresponding copyable texture handle
  handle: TextureHandle,

  /// The offset of the allocation in the atlas, in pixels
  offset: UVec2,

  /// The requested size of the allocation, in pixels
  size: UVec2,

  /// The maximum size of the allocation, used for reusing deallocated allocations
  ///
  /// Usually equal to `size`, but may be larger than the requested size
  /// if the allocation was reused by a smaller texture at some point
  max_size: UVec2,
}

impl TextureAllocation {
  /// Create a new texture allocation with the specified parameters.
  ///
  /// The `max_size` parameter will be set equal to `size`.
  pub fn new(handle: TextureHandle, offset: UVec2, size: UVec2) -> Self {
    Self {
      handle,
      offset,
      size,
      max_size: size,
    }
  }
}

/// A texture atlas that can be used to pack multiple textures into a single texture.
pub struct TextureAtlas {
  /// The size of the atlas, in pixels
  size: UVec2,

  /// The texture data of the atlas, ALWAYS in RGBA8 format
  data: Vec<u8>,

  /// The packer used to allocate space for textures in the atlas
  packer: DensePacker,

  /// The next id to be used for a texture handle\
  /// Gets incremented every time a new texture is allocated
  next_id: TextureId,

  /// Active allocated textures, indexed by id of their handle
  allocations: HashMap<TextureId, TextureAllocation, BuildNoHashHasher<TextureId>>,

  /// Deallocated allocations that can be reused, sorted by size
  //TODO: use binary heap or btreeset for reuse_allocations instead, but this works for now
  reuse_allocations: Vec<TextureAllocation>,
}

impl TextureAtlas {
  /// Create a new texture atlas with the specified size.
  pub(crate) fn new(size: UVec2) -> Self {
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
      reuse_allocations: Vec::new(),
    }
  }

  /// Get the next handle
  ///
  /// Does not allocate a texture associated with it
  /// This handle will be invalid until it's associated with a texture.
  ///
  /// Used internally in `allocate` and `allocate_with_data`.
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

    // Check if any deallocated allocations can be reused
    // Find the smallest allocation that fits the requested size
    // (The list is already sorted by size)
    for (idx, allocation) in self.reuse_allocations.iter().enumerate() {
      if allocation.max_size.x >= size.x && allocation.max_size.y >= size.y {
        let allocation = self.reuse_allocations.remove(idx);
        let handle = self.next_handle(size);
        self.allocations.insert_unique_unchecked(handle.id, TextureAllocation {
          handle,
          offset: allocation.offset,
          size,
          max_size: allocation.max_size,
        });
        return handle;
      }
    }

    // Pack the texture
    let pack = self.packer.pack(
      size.x as i32,
      size.y as i32,
      false
    );

    //TODO: handle pack failure by either resizing the atlas or returning an error
    let pack = pack.unwrap();
    let offset = ivec2(pack.x, pack.y).as_uvec2();

    // Allocate the texture
    let handle = self.next_handle(size);
    let allocation = TextureAllocation::new(handle, offset, size);
    self.allocations.insert_unique_unchecked(handle.id, allocation);

    handle
  }

  /// Deallocate a texture in the atlas, allowing its space to be reused by future allocations.
  ///
  /// # Panics
  /// - If the texture handle is invalid for this atlas.
  pub fn deallocate(&mut self, handle: TextureHandle) {
    // Remove the allocation from the active allocations
    let allocation = self.allocations
      .remove(&handle.id)
      .expect("invalid texture handle");

    // TODO: this is not the most efficient way to do this:
    // And put it in the reuse allocations queue
    self.reuse_allocations.push(allocation);
    self.reuse_allocations.sort_unstable_by_key(|a| a.size.x * a.size.y);
  }

  /// Update the data of a texture in the atlas.\
  /// The texture must have been previously allocated with `allocate` or `allocate_with_data`.
  ///
  /// The source data must be in the format specified by the `format` parameter.\
  /// (Please note that the internal format of the texture is always RGBA8, regardless of the source format.)
  ///
  /// The function will silently ignore any data that doesn't fit in the texture.
  ///
  /// # Panics
  /// - If the texture handle is invalid for this atlas.
  /// - The length of the data array is less than the size of the texture.
  pub fn update(&mut self, handle: TextureHandle, format: SourceTextureFormat, data: &[u8]) {
    assert!(
      data.len() >= handle.size.x as usize * handle.size.y as usize * format.bytes_per_pixel(),
      "data length must be at least the size of the texture"
    );

    let bpp = format.bytes_per_pixel();

    let TextureAllocation { size, offset, ..} = self.allocations
      .get(&handle.id)
      .expect("invalid texture handle");

    for y in 0..size.y {
      for x in 0..size.x {
        let src_idx = (y * size.x + x) as usize * bpp;
        let dst_idx: usize = (
          (offset.y + y) * size.x +
          (offset.x + x)
        ) as usize * RGBA_BYTES_PER_PIXEL;

        let src = &data[src_idx..src_idx + bpp];
        let dst = &mut self.data[dst_idx..dst_idx + RGBA_BYTES_PER_PIXEL];

        match format {
          SourceTextureFormat::RGBA8 => {
            dst.copy_from_slice(src);
          },
          SourceTextureFormat::ARGB8 => {
            dst[..3].copy_from_slice(&src[1..]);
            dst[3] = src[0];
          },
          SourceTextureFormat::BGRA8 => {
            dst.copy_from_slice(src);
            dst.rotate_right(1);
            dst.reverse();
          },
          SourceTextureFormat::ABGR8 => {
            dst.copy_from_slice(src);
            dst.reverse();
          },
          SourceTextureFormat::RGB8 => {
            dst[..3].copy_from_slice(src);
            dst[3] = 0xff;
          },
          SourceTextureFormat::BGR8 => {
            dst[..3].copy_from_slice(src);
            dst[..3].reverse();
            dst[3] = 0xff;
          },
          SourceTextureFormat::A8 => {
            dst[..3].fill(0xff);
            dst[3] = src[0];
          },
        }
      }
    }
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

    // Write the data to the texture
    self.update(handle, format, data);

    handle
  }
}

impl Default for TextureAtlas {
  fn default() -> Self {
    Self::new(DEFAULT_ATLAS_SIZE)
  }
}
