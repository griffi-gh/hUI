use glam::{uvec2, vec2, UVec2, Vec2};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use rect_packer::DensePacker;
use crate::rectangle::Corners;

const RGBA_CHANNEL_COUNT: u32 = 4;
const ALLOW_ROTATION: bool = false;

pub struct TextureAtlasMeta<'a> {
  pub data: &'a [u8],
  pub size: UVec2,
  pub modified: bool,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct TextureHandle {
  //pub(crate) rc: Rc<()>,
  pub(crate) index: u32
}

#[derive(Clone, Debug)]
pub(crate) struct TextureAllocation {
  //pub(crate) rc: Weak<()>,

  /// Unique index of the texture allocation
  pub index: u32,

  /// Position in the texture atlas
  pub(crate) position: UVec2,

  /// Requested texture size
  pub size: UVec2,

  /// True if the texture was rotated by 90 degrees
  pub(crate) rotated: bool,
}

/// Manages a texture atlas and the allocation of space within it\
/// The atlas is alllowed to grow and resize dynamically, as needed
pub(crate) struct TextureAtlasManager {
  packer: DensePacker,
  count: u32,
  size: UVec2,
  data: Vec<u8>,
  allocations: HashMap<u32, TextureAllocation, BuildNoHashHasher<u32>>,
  /// Items that have been removed from the allocation list, but still affect
  remove_queue: Vec<TextureAllocation>,
  /// True if the atlas has been modified in a way which requires a texture reupload
  /// since the beginning of the current frame
  modified: bool,
}

impl TextureAtlasManager {
  /// Create a new texture atlas with the specified size\
  /// 512x512 is a good default size for most applications, and the texture atlas can grow dynamically as needed
  pub fn new(size: UVec2) -> Self {
    Self {
      packer: DensePacker::new(size.x as i32, size.y as i32),
      count: 0,
      size,
      data: vec![0; (size.x * size.y * RGBA_CHANNEL_COUNT) as usize],
      allocations: HashMap::default(),
      remove_queue: Vec::new(),
      modified: true,
    }
  }

  /// Resize the texture atlas to the new size in-place, preserving the existing data
  pub fn resize(&mut self, new_size: UVec2) {
    log::trace!("resizing texture atlas to {:?}", new_size);
    if self.size == new_size {
      log::warn!("Texture atlas is already the requested size");
      return
    }
    if new_size.x > self.size.x && new_size.y > self.size.y{
      self.packer.resize(new_size.x as i32, new_size.y as i32);
      //Resize the data array in-place
      self.data.resize((new_size.x * new_size.y * RGBA_CHANNEL_COUNT) as usize, 0);
      for y in (1..self.size.y).rev() {
        for x in (0..self.size.x).rev() {
          let idx = ((y * self.size.x + x) * RGBA_CHANNEL_COUNT) as usize;
          let new_idx = ((y * new_size.x + x) * RGBA_CHANNEL_COUNT) as usize;
          for c in 0..(RGBA_CHANNEL_COUNT as usize) {
            self.data[new_idx + c] = self.data[idx + c];
          }
        }
      }
    } else {
      //If scaling down, just recreate the atlas from scratch (since we need to re-pack everything anyway)
      todo!("Atlas downscaling is not implemented yet");
    }
    self.size = new_size;
    self.modified = true;
  }

  /// Ensure that a texture with specified size would fit without resizing on the next allocation attempt\
  pub fn ensure_fits(&mut self, size: UVec2) {
    // Plan A: try if any of the existing items in the remove queue would fit the texture
    // Plan B: purge the remove queue, recreate the packer and try again (might be expensive...!)
    // TODO: implement these
    // Plan C: resize the atlas
    let mut new_size = self.size;
    while !self.packer.can_pack(size.x as i32, size.y as i32, true) {
      new_size *= 2;
      self.packer.resize(new_size.x as i32, new_size.y as i32);
    }
    if new_size != self.size {
      self.resize(new_size);
    }
  }

  /// Allocate a new texture region in the atlas and return a handle to it\
  /// Returns None if the texture could not be allocated due to lack of space\
  /// Use `allocate` to allocate a texture and resize the atlas if necessary\
  /// Does not modify the texture data
  fn try_allocate(&mut self, size: UVec2) -> Option<TextureHandle> {
    log::trace!("Allocating texture of size {:?}", size);
    let result = self.packer.pack(size.x as i32, size.y as i32, ALLOW_ROTATION)?;
    let index = self.count;
    self.count += 1;
    let allocation = TextureAllocation {
      index,
      position: UVec2::new(result.x as u32, result.y as u32),
      size,
      //If the size does not match the requested size, the texture was rotated
      rotated: ALLOW_ROTATION && (result.width != size.x as i32),
    };
    self.allocations.insert_unique_unchecked(index, allocation);
    Some(TextureHandle { index })
  }

  /// Allocate a new texture region in the atlas and resize the atlas if necessary\
  /// This function should never fail under normal circumstances.\
  /// May modify the texture data if the atlas is resized
  pub fn allocate(&mut self, size: UVec2) -> TextureHandle {
    self.ensure_fits(size);
    self.try_allocate(size).unwrap()
  }

  /// Allocate a new texture region in the atlas and copy the data into it\
  /// This function may resize the atlas as needed, and should never fail under normal circumstances.
  pub fn add(&mut self, width: usize, data: &[u8]) -> TextureHandle {
    let size = uvec2(width as u32, (data.len() / (width * RGBA_CHANNEL_COUNT as usize)) as u32);
    let handle: TextureHandle = self.allocate(size);
    let allocation = self.allocations.get(&handle.index).unwrap();
    assert!(!allocation.rotated, "Rotated textures are not implemented yet");
    for y in 0..size.y {
      for x in 0..size.x {
        let src_idx = (y * size.x + x) * RGBA_CHANNEL_COUNT;
        let dst_idx = ((allocation.position.y + y) * self.size.x + allocation.position.x + x) * RGBA_CHANNEL_COUNT;
        for c in 0..RGBA_CHANNEL_COUNT as usize {
          self.data[dst_idx as usize + c] = data[src_idx as usize + c];
        }
      }
    }
    self.modified = true;
    handle
  }

  /// Works the same way as [`TextureAtlasManager::add`], but the input data is assumed to be grayscale (1 channel per pixel)\
  /// The data is copied into the alpha channel of the texture, while all the other channels are set to 255\
  /// May resize the atlas as needed, and should never fail under normal circumstances.
  pub fn add_grayscale(&mut self, width: usize, data: &[u8]) -> TextureHandle {
    let size = uvec2(width as u32, (data.len() / width) as u32);
    let handle = self.allocate(size);
    let allocation = self.allocations.get(&handle.index).unwrap();
    assert!(!allocation.rotated, "Rotated textures are not implemented yet");
    for y in 0..size.y {
      for x in 0..size.x {
        let src_idx = (y * size.x + x) as usize;
        let dst_idx = (((allocation.position.y + y) * self.size.x + allocation.position.x + x) * RGBA_CHANNEL_COUNT) as usize;
        self.data[dst_idx..(dst_idx + RGBA_CHANNEL_COUNT as usize)].copy_from_slice(&[255, 255, 255, data[src_idx]]);
      }
    }
    self.modified = true;
    handle
  }

  pub fn modify(&mut self, handle: TextureHandle) {
    todo!()
  }

  pub fn remove(&mut self, handle: TextureHandle) {
    todo!()
  }

  pub fn get(&self, handle: TextureHandle) -> Option<&TextureAllocation> {
    self.allocations.get(&handle.index)
  }

  pub(crate) fn get_uv(&self, handle: TextureHandle) -> Option<Corners<Vec2>> {
    let info = self.get(handle)?;
    let atlas_size = self.meta().size.as_vec2();
    let p0x = info.position.x as f32 / atlas_size.x;
    let p1x = (info.position.x as f32 + info.size.x as f32) / atlas_size.x;
    let p0y = info.position.y as f32 / atlas_size.y;
    let p1y = (info.position.y as f32 + info.size.y as f32) / atlas_size.y;
    Some(Corners {
      top_left: vec2(p0x, p0y),
      top_right: vec2(p1x, p0y),
      bottom_left: vec2(p0x, p1y),
      bottom_right: vec2(p1x, p1y),
    })
  }

  /// Reset the `is_modified` flag
  pub(crate) fn reset_modified(&mut self) {
    self.modified = false;
  }

  /// Returns true if the atlas has been modified since the beginning of the current frame\
  /// If this function returns true, the texture atlas should be re-uploaded to the GPU before rendering\
  pub fn is_modified(&self) -> bool {
    self.modified
  }

  pub fn meta(&self) -> TextureAtlasMeta {
    TextureAtlasMeta {
      data: &self.data,
      size: self.size,
      modified: self.modified,
    }
  }
}

impl Default for TextureAtlasManager {
  /// Create a new texture atlas with a default size of 512x512
  fn default() -> Self {
    Self::new(UVec2::new(512, 512))
  }
}
