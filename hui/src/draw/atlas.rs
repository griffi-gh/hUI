use glam::UVec2;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use rect_packer::DensePacker;

const CHANNEL_COUNT: u32 = 4;

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

pub(crate) struct TextureAtlasManager {
  packer: DensePacker,
  count: u32,
  size: UVec2,
  data: Vec<u8>,
  allocations: HashMap<u32, TextureAllocation, BuildNoHashHasher<u32>>,
}

impl TextureAtlasManager {
  pub fn new(size: UVec2) -> Self {
    Self {
      packer: DensePacker::new(size.x as i32, size.y as i32),
      count: 0,
      size: UVec2::new(0, 0),
      data: Vec::new(),
      allocations: HashMap::default(),
    }
  }

  pub fn resize(&mut self, new_size: UVec2) {
    if new_size.x > self.size.x && new_size.y > self.size.y{
      self.packer.resize(new_size.x as i32, new_size.y as i32);
      //Resize the data array in-place
      self.data.resize((new_size.x * new_size.y * CHANNEL_COUNT) as usize, 0);
      for y in (1..self.size.y).rev() {
        for x in (0..self.size.x).rev() {
          let idx = (y * self.size.x + x) as usize;
          let new_idx = (y * new_size.x + x) as usize;
          self.data[new_idx] = self.data[idx];
        }
      }
    } else {
      //If scaling down, just recreate the atlas from scratch (since we need to re-pack everything anyway)
      todo!("Atlas downscaling is not implemented yet");
    }
    self.size = new_size;
  }

  /// Allocate a new texture region in the atlas
  pub fn allocate(&mut self, size: UVec2) -> Option<TextureHandle> {
    let result = self.packer.pack(size.x as i32, size.y as i32, true)?;
    let index = self.count;
    self.count += 1;
    let allocation = TextureAllocation {
      index,
      position: UVec2::new(result.x as u32, result.y as u32),
      size,
      //If the size does not match the requested size, the texture was rotated
      rotated: result.width != size.x as i32,
    };
    self.allocations.insert_unique_unchecked(index, allocation);
    Some(TextureHandle { index })
  }

  /// Allocate a new texture region in the atlas and copy the data into it
  pub fn add(&mut self, width: u32, data: &[u8]) {
    todo!()
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
}

impl Default for TextureAtlasManager {
  fn default() -> Self {
    Self::new(UVec2::new(512, 512))
  }
}
