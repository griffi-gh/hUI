use alloc::vec::Vec;
use glam::{ivec2, uvec2, vec2, UVec2, Vec2};
use hui_shared::rect::Corners;
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
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct TextureHandle {
  pub(crate) id: TextureId,
  pub(crate) size: UVec2,
}

impl TextureHandle {
  /// Create a new broken texture handle.
  pub fn new_broken() -> Self {
    Self {
      id: u32::MAX,
      size: uvec2(0, 0),
    }
  }

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

#[derive(Clone, Copy)]
pub struct TextureAtlasBackendData<'a> {
  pub data: &'a [u8],
  pub size: UVec2,
  pub version: u64,
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

  /// Version of the texture atlas, incremented every time the atlas is modified
  version: u64,
}

impl TextureAtlas {
  /// Internal function, only directly used in tests
  pub(crate) fn new_internal(size: UVec2) -> Self {
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
      version: 0,
    }
  }

  /// Create a new texture atlas with the specified size.
  pub(crate) fn new(size: UVec2) -> Self {
    let mut this = Self::new_internal(size);

    // HACK?: ensure 0,0 is a white pixel
    let h = this.add_with_data(SourceTextureFormat::A8, &[255], 1);
    debug_assert!(
      h.size == uvec2(1, 1) && h.id == 0,
      "The texture handle was not allocated correctly"
    );
    debug_assert!(
      this.get_uv(h).is_some_and(|x| x.top_left == Vec2::ZERO),
      "The texture was't allocated in the top-left corner"
    );

    this
  }

  /// The version of the atlas, incremented every time the atlas is modified.
  pub fn version(&self) -> u64 {
    self.version
  }

  /// The underlying texture data of the atlas, in RGBA8 format.
  pub fn data_rgba(&self) -> &[u8] {
    &self.data
  }

  /// Get data needed by the backend implementation.
  pub fn backend_data(&self) -> TextureAtlasBackendData {
    TextureAtlasBackendData {
      data: &self.data,
      size: self.size,
      version: self.version,
    }
  }

  /// Increment the version of the atlas
  fn increment_version(&mut self) {
    // XXX: wrapping_add? will this *ever* overflow?
    self.version = self.version.wrapping_add(1);
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

  /// Returns the next size the canvas should be resized to in case we run out of space
  pub(crate) fn next_size(size: UVec2) -> UVec2 {
    size * if size.x >= size.y {
      uvec2(1, 2)
    } else {
      uvec2(2, 1)
    }
  }

  // TODO resize test

  /// Resize the atlas to the specified size
  ///
  /// Downscaling is not supported.
  pub(crate) fn resize(&mut self, new_size: UVec2) {
    if new_size == self.size {
      return
    }
    assert_size(new_size);
    assert!(
      new_size.x >= self.size.x &&
      new_size.y >= self.size.y,
      "downscaling is not supported"
    );

    let old_size = self.size;
    self.size = new_size;

    log::debug!("resize canvas {old_size} -> {new_size}");

    if self.packer.size() != (new_size.x as i32, new_size.y as i32) {
      self.packer.resize(new_size.x as i32, new_size.y as i32);
    }

    let new_data_len = (new_size.y as usize * new_size.x as usize) * RGBA_BYTES_PER_PIXEL;
    self.data.resize(new_data_len, 0);

    // Resize the atlas data in-place if needed
    if new_size.x != old_size.x {
      for y in (1..old_size.y).rev() { // First source row can be skipped (its alr at idx 0)
        for x in (0..old_size.x).rev() {
          let old_idx = (y as usize * old_size.x as usize + x as usize) * RGBA_BYTES_PER_PIXEL;
          let new_idx = (y as usize * new_size.x as usize + x as usize) * RGBA_BYTES_PER_PIXEL;
          self.data.copy_within(old_idx..old_idx + RGBA_BYTES_PER_PIXEL, new_idx);
        }
      }
    }

    self.increment_version();
  }

  /// Allocate a texture in the atlas, returning a handle to it.\
  /// The data present in the texture is undefined, and may include garbage data.
  ///
  /// The texture may be resized if the texture doesn't fit
  ///
  /// # Panics
  /// - If any of the dimensions of the texture are zero or exceed `i32::MAX`.
  pub fn add_empty(&mut self, size: UVec2) -> TextureHandle {
    assert_size(size);

    // Check if any deallocated allocations can be reused
    // Find the smallest allocation that fits the requested size
    // (The list is already sorted by size)
    for (idx, allocation) in self.reuse_allocations.iter().enumerate() {
      if allocation.max_size.x >= size.x && allocation.max_size.y >= size.y {
        let allocation = self.reuse_allocations.remove(idx);
        let handle = self.next_handle(size);
        unsafe {
          self.allocations.insert_unique_unchecked(handle.id, TextureAllocation {
            handle,
            offset: allocation.offset,
            size,
            max_size: allocation.max_size,
          });
        }
        return handle;
      }
    }

    let mut new_size = self.size;
    while !self.packer.can_pack(
      size.x as i32,
      size.y as i32,
      false
    ) {
      new_size = Self::next_size(new_size);
      log::trace!("need to resize, is {new_size} enough?");
      self.packer.resize(new_size.x as i32, new_size.y as i32);
    }
    if new_size != self.size {
      self.resize(new_size);
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
    unsafe {
      self.allocations.insert_unique_unchecked(handle.id, allocation);
    }

    handle
  }

  /// Deallocate a texture in the atlas, allowing its space to be reused by future allocations.
  ///
  /// # Panics
  /// - If the texture handle is invalid for this atlas.
  pub fn remove(&mut self, handle: TextureHandle) {
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

    debug_assert_eq!(*size, handle.size, "texture size mismatch");

    for y in 0..size.y {
      for x in 0..size.x {
        let src_idx = (y * size.x + x) as usize * bpp;
        let dst_idx: usize = (
          (offset.y + y) * self.size.x +
          (offset.x + x)
        ) as usize * RGBA_BYTES_PER_PIXEL;

        let src = &data[src_idx..src_idx + bpp];
        let dst = &mut self.data[dst_idx..dst_idx + RGBA_BYTES_PER_PIXEL];

        match format {
          SourceTextureFormat::RGBA8 => {
            // TODO opt: copy entire row in this case
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

    self.increment_version();
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
  pub fn add_with_data(&mut self, format: SourceTextureFormat, data: &[u8], width: usize) -> TextureHandle {
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
    let handle = self.add_empty(size);

    // Write the data to the texture
    self.update(handle, format, data);

    handle
  }

  /// Get uv coordinates for the texture handle.
  pub(crate) fn get_uv(&self, handle: TextureHandle) -> Option<Corners<Vec2>> {
    let TextureAllocation { offset, size, .. } = self.allocations
      .get(&handle.id)?;
    let p0x = offset.x as f32 / self.size.x as f32;
    let p1x = (offset.x as f32 + size.x as f32) / self.size.x as f32;
    let p0y = offset.y as f32 / self.size.y as f32;
    let p1y = (offset.y as f32 + size.y as f32) / self.size.y as f32;
    Some(Corners {
      top_left: vec2(p0x, p0y),
      top_right: vec2(p1x, p0y),
      bottom_left: vec2(p0x, p1y),
      bottom_right: vec2(p1x, p1y),
    })
  }
}

impl Default for TextureAtlas {
  fn default() -> Self {
    Self::new(DEFAULT_ATLAS_SIZE)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_assert_size_valid() {
    assert_size(uvec2(1, 1));
    assert_size(uvec2(i32::MAX as u32, i32::MAX as u32));
  }

  #[test]
  #[should_panic(expected = "size must be greater than 0")]
  fn test_assert_size_zero() {
    assert_size(uvec2(0, 0));
  }

  #[test]
  #[should_panic(expected = "size must be less than i32::MAX")]
  fn test_assert_size_too_large() {
    assert_size(uvec2(i32::MAX as u32 + 1, i32::MAX as u32 + 1));
  }

  #[test]
  fn test_texture_handle_new_broken() {
    let handle = TextureHandle::new_broken();
    assert_eq!(handle.id, u32::MAX);
    assert_eq!(handle.size, uvec2(0, 0));
  }

  #[test]
  fn test_texture_allocation_new() {
    let handle = TextureHandle::new_broken();
    let allocation = TextureAllocation::new(handle, uvec2(1, 1), uvec2(2, 2));
    assert_eq!(allocation.handle, handle);
    assert_eq!(allocation.offset, uvec2(1, 1));
    assert_eq!(allocation.size, uvec2(2, 2));
    assert_eq!(allocation.max_size, uvec2(2, 2));
  }

  #[test]
  fn test_texture_atlas_new() {
    const SIZE: u32 = 128;

    let atlas = TextureAtlas::new_internal(uvec2(SIZE, SIZE));
    assert_eq!(atlas.size, uvec2(SIZE, SIZE));
    assert_eq!(atlas.data.len(), (SIZE as usize) * (SIZE as usize) * RGBA_BYTES_PER_PIXEL);
    assert_eq!(atlas.next_id, 0);
    assert_eq!(atlas.allocations.len(), 0);
    assert_eq!(atlas.reuse_allocations.len(), 0);
    assert_eq!(atlas.version, 0);
  }

  #[test]
  fn test_texture_atlas_add_empty() {
    let mut atlas = TextureAtlas::new_internal(uvec2(128, 128));
    let handle = atlas.add_empty(uvec2(32, 32));
    assert_eq!(handle.size, uvec2(32, 32));
    assert_eq!(atlas.get_uv(handle).unwrap().bottom_right, vec2(32. / 128., 32. / 128.));
    assert_eq!(atlas.allocations.len(), 1);
  }

  #[test]
  fn test_texture_atlas_add_with_data() {
    fn make_data(o: u8)-> Vec<u8> {
      let mut data = vec![o; 32 * 32 * 4];
      for y in 0..32 {
        for x in 0..32 {
          let idx = (y * 32 + x) * 4;
          data[idx] = x as u8;
          data[idx + 1] = y as u8;
        }
      }
      data
    }

    let mut atlas = TextureAtlas::new_internal(uvec2(128, 128));

    let data = make_data(1);
    let handle = atlas.add_with_data(SourceTextureFormat::RGBA8, &data, 32);
    assert_eq!(handle.size, uvec2(32, 32));
    assert_eq!(atlas.allocations.len(), 1);
    let uv = atlas.get_uv(handle).unwrap();
    assert_eq!(uv.top_left, vec2(0.0, 0.0));
    assert_eq!(uv.top_right, vec2(32.0 / 128.0, 0.0));
    assert_eq!(uv.bottom_left, vec2(0.0, 32.0 / 128.0));
    assert_eq!(uv.bottom_right, vec2(32.0 / 128.0, 32.0 / 128.0));

    let data = make_data(2);
    let handle = atlas.add_with_data(SourceTextureFormat::RGBA8, &data, 32);
    assert_eq!(handle.size, uvec2(32, 32));
    assert_eq!(atlas.allocations.len(), 2);
    let uv = atlas.get_uv(handle).unwrap();
    assert_eq!(uv.top_left, vec2(32.0 / 128.0, 0.0));
    assert_eq!(uv.top_right, vec2(64.0 / 128.0, 0.0));
    assert_eq!(uv.bottom_left, vec2(32.0 / 128.0, 32.0 / 128.0));
    assert_eq!(uv.bottom_right, vec2(64.0 / 128.0, 32.0 / 128.0));

    // now, check the texture data
    assert_eq!(atlas.version(), 2);
    let data = atlas.data_rgba();

    // for y in 0..128 {
    //   for x in 0..128 {
    //     let idx = (y * 128 + x) * 4;
    //     print!("{}", data[idx + 2]);
    //   }
    //   println!();
    // }

    for y in 0..128 {
      for x in 0..128 {
        let idx = (y * 128 + x) * 4;
        if y >= 32 || x >= 64 {
          continue
        }
        assert_eq!(
          if x < 32 {
            [x as u8, y as u8, 1, 1]
          } else if x < 64 {
            [x as u8 - 32, y as u8, 2, 2]
          } else {
            unreachable!()
          },
          data[idx..idx + 4],
          "pixel at ({x}, {y}) idx: {idx} is incorrect",
        );
      }
    }
  }

  // #[test]
  // fn test_texture_atlas_update() {
  //   let mut atlas = TextureAtlas::new(uvec2(128, 128));
  //   let data = vec![255; 32 * 32 * 4];
  //   let handle = atlas.allocate_with_data(SourceTextureFormat::RGBA8, &data, 32);
  //   let new_data = vec![0; 32 * 32 * 4];
  //   atlas.update(handle, SourceTextureFormat::RGBA8, &new_data);
  //   assert_eq!(atlas.data_rgba()[..32 * 32 * 4], new_data[..]);
  // }

  #[test]
  fn test_texture_atlas_remove() {
    let mut atlas = TextureAtlas::new_internal(uvec2(128, 128));
    let handle = atlas.add_empty(uvec2(32, 32));
    atlas.remove(handle);
    assert_eq!(atlas.allocations.len(), 0);
    assert_eq!(atlas.reuse_allocations.len(), 1);
  }

  #[test]
  fn test_texture_atlas_get_uv() {
    let mut atlas = TextureAtlas::new_internal(uvec2(128, 128));
    let handle = atlas.add_empty(uvec2(32, 32));
    let uv = atlas.get_uv(handle).unwrap();
    assert_eq!(uv.top_left, vec2(0.0, 0.0));
    assert_eq!(uv.top_right, vec2(32.0 / 128.0, 0.0));
    assert_eq!(uv.bottom_left, vec2(0.0, 32.0 / 128.0));
    assert_eq!(uv.bottom_right, vec2(32.0 / 128.0, 32.0 / 128.0));
  }
}