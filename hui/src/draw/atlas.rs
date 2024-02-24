use glam::{uvec2, UVec2};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use rect_packer::DensePacker;

use crate::IfModified;

const CHANNEL_COUNT: u32 = 4;
//TODO: make this work
const ALLOW_ROTATION: bool = false;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureHandle {
  //TODO automatic cleanup when handle is dropped
  //man: Weak<RefCell<TextureAtlasManager>>,
  pub(crate) index: u32
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextureAllocation {
  /// Index of the texture allocation
  pub index: u32,

  /// Position in the texture atlas
  pub position: UVec2,

  /// Requested texture size
  pub size: UVec2,

  /// True if the texture was rotated by 90 degrees
  pub rotated: bool,
}

/// Manages a texture atlas and the allocation of space within it\
/// The atlas is alllowed to grow and resize dynamically, as needed
pub(crate) struct TextureAtlasManager {
  packer: DensePacker,
  count: u32,
  size: UVec2,
  data: Vec<u8>,
  allocations: HashMap<u32, TextureAllocation, BuildNoHashHasher<u32>>,
  modified: bool,
}

impl TextureAtlasManager {
  /// Create a new texture atlas with the specified size\
  /// 512x512 is a good default size for most applications, and the texture atlas can grow dynamically as needed
  pub fn new(size: UVec2) -> Self {
    Self {
      packer: DensePacker::new(size.x as i32, size.y as i32),
      count: 0,
      size: UVec2::new(0, 0),
      data: vec![0; (size.x * size.y * CHANNEL_COUNT) as usize],
      allocations: HashMap::default(),
      modified: true,
    }
  }

  /// Resize the texture atlas to the new size in-place, preserving the existing data
  pub fn resize(&mut self, new_size: UVec2) {
    if new_size.x > self.size.x && new_size.y > self.size.y{
      self.packer.resize(new_size.x as i32, new_size.y as i32);
      //Resize the data array in-place
      self.data.resize((new_size.x * new_size.y * CHANNEL_COUNT) as usize, 0);
      for y in (1..self.size.y).rev() {
        for x in (0..self.size.x).rev() {
          let idx = ((y * self.size.x + x) * CHANNEL_COUNT) as usize;
          let new_idx = ((y * new_size.x + x) * CHANNEL_COUNT) as usize;
          for c in 0..(CHANNEL_COUNT as usize) {
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

  /// Allocate a new texture region in the atlas and return a handle to it\
  /// Returns None if the texture could not be allocated due to lack of space\
  /// Use `allocate` to allocate a texture and resize the atlas if necessary\
  /// Does not modify the texture data
  fn try_allocate(&mut self, size: UVec2) -> Option<TextureHandle> {
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
    let mut new_size = self.size;
    while !self.packer.can_pack(size.x as i32, size.y as i32, true) {
      new_size *= 2;
      self.packer.resize(new_size.x as i32, new_size.y as i32);
    }
    self.resize(new_size);
    self.try_allocate(size).unwrap()
  }

  /// Allocate a new texture region in the atlas and copy the data into it\
  /// This function may resize the atlas as needed, and should never fail under normal circumstances.
  pub fn add(&mut self, width: usize, data: &[u8]) -> TextureHandle {
    let size = uvec2(width as u32, (data.len() / (width * CHANNEL_COUNT as usize)) as u32);
    let handle = self.allocate(size);
    let allocation = self.allocations.get_mut(&handle.index).unwrap();
    assert!(!allocation.rotated, "Rotated textures are not implemented yet");
    for y in 0..size.y {
      for x in 0..size.x {
        let src_idx = (y * size.x + x) * CHANNEL_COUNT;
        let dst_idx = ((allocation.position.y + y) * self.size.x + allocation.position.x + x) * CHANNEL_COUNT;
        for c in 0..CHANNEL_COUNT as usize {
          self.data[dst_idx as usize + c] = data[src_idx as usize + c];
        }
      }
    }
    handle
  }

  pub fn modify(&mut self, handle: TextureHandle) {
    todo!()
  }

  pub fn remove(&mut self, handle: TextureHandle) {
    todo!()
  }

  pub fn atlas_size(&self) -> UVec2 {
    self.size
  }

  pub fn get(&self, handle: TextureHandle) -> Option<&TextureAllocation> {
    self.allocations.get(&handle.index)
  }

  /// Reset the `is_modified` flag
  pub fn reset_modified(&mut self) {
    self.modified = false;
  }

  /// Returns true if the atlas has been modified since the last call to `reset_modified`\
  /// If this function returns true, the texture atlas should be re-uploaded to the GPU\
  /// This function is mostly useful for developers of graphics backends
  pub fn is_modified(&self) -> bool {
    self.modified
  }
}

impl Default for TextureAtlasManager {
  /// Create a new texture atlas with a default size of 512x512
  fn default() -> Self {
    Self::new(UVec2::new(512, 512))
  }
}

#[allow(deprecated)]
impl<'a> IfModified<TextureAtlasManager> for TextureAtlasManager {
  fn if_modified(&self) -> Option<&Self> {
    match self.modified {
      true => Some(self),
      false => None,
    }
  }
}
