use hui_painter::{
  PainterInstance,
  paint::command::PaintList,
  text::FontHandle,
  texture::{SourceTextureFormat, TextureHandle},
};
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{Direction, LayoutInfo},
  signal::{Signal, SignalStore},
  event::{EventQueue, UiEvent},
  input::UiInputState,
  rect::Rect,
  state::StateRepo,
};

pub struct RenderInfo<'a> {
  pub id: u64,
  pub list: &'a PaintList,
}

/// The main instance of the UI system.
///
/// In most cases, you should only have one instance of this struct, but multiple instances are allowed\
/// (Please note that it's possible to render multiple UI "roots" using a single instance)
pub struct UiInstance {
  stateful_state: StateRepo,
  events: EventQueue,
  input: UiInputState,
  signal: SignalStore,

  painter: PainterInstance,
  draw_commands: PaintList,
  prev_draw_commands: PaintList,
  draw_call_id: u64,

  /// True if in the middle of a laying out a frame
  state: bool,
}

impl UiInstance {
  /// Crate and initialize a new instance of the UI
  ///
  /// In most cases, you should only do this *once*, during the initialization of your application
  pub fn new() -> Self {
    UiInstance {
      //mouse_position: Vec2::ZERO,
      stateful_state: StateRepo::new(),
      painter: PainterInstance::new(),

      draw_commands: PaintList::new_empty(),
      prev_draw_commands: PaintList::new_empty(),
      draw_call_id: 0,

      events: EventQueue::new(),
      input: UiInputState::new(),
      signal: SignalStore::new(),
      state: false,
    }
  }

  /// Parse and add a font from a raw byte slice to the UI\
  /// TrueType (`.ttf`/`.ttc`) and OpenType (`.otf`) fonts are supported\
  ///
  /// Returns a font handle ([`FontHandle`]).
  ///
  /// ## Panics:
  /// If the font data is invalid or corrupt
  #[deprecated(since = "0.1.0-alpha.5", note = "Use painter.fonts.add_font() instead")]
  pub fn add_font(&mut self, font: &[u8]) -> FontHandle {
    self.painter.fonts.add(font)
  }

  /// Add an image to the texture atlas\
  /// Accepted texture formats are `Rgba` and `Grayscale`
  ///
  /// Returns an image handle ([`ImageHandle`])\
  /// This handle can be used to reference the texture in draw commands\
  /// It's a light reference and can be cloned/copied freely, but will not be cleaned up even when dropped
  #[deprecated(since = "0.1.0-alpha.5", note = "Use painter.atlas.allocate_with_data() instead")]
  pub fn add_image(&mut self, format: SourceTextureFormat, data: &[u8], width: usize) -> TextureHandle {
    self.painter.atlas.allocate_with_data(format, data, width)
  }

  //TODO better error handling

  /// ## DEPRECATED: This method will be removed in the future
  ///
  /// ---
  ///
  /// Add an image from a file to the texture atlas\
  ///
  /// Requires the `image` feature
  ///
  /// # Panics:
  /// - If the file exists but contains invalid image data\
  ///   (this will change to a soft error in the future)
  #[cfg(feature = "image")]
  #[deprecated(since = "0.1.0-alpha.5", note = "Will be removed in the future in favor of modular image loading in hui-painter")]
  pub fn add_image_file_path(&mut self, path: impl AsRef<std::path::Path>) -> Result<TextureHandle, std::io::Error> {
    use std::io::{Read, Seek};

    // Open the file (and wrap it in a bufreader)
    let mut file = std::io::BufReader::new(std::fs::File::open(path)?);

    //Guess the image format from the magic bytes
    //Read like 64 bytes, which should be enough for magic byte detection
    //well this would fail if the image is somehow smaller than 64 bytes, but who the fvck cares...
    let mut magic = [0; 64];
    file.read_exact(&mut magic)?;
    let format = image::guess_format(&magic).expect("Invalid image data (FORMAT)");
    file.seek(std::io::SeekFrom::Start(0))?;

    //Parse the image and read the raw uncompressed rgba data
    let image = image::load(file, format).expect("Invalid image data");
    let image_rgba = image.as_rgba8().unwrap();

    //Add the image to the atlas

    let handle = self.painter.atlas.allocate_with_data(
      SourceTextureFormat::RGBA8,
      image_rgba,
      image.width() as usize
    );

    Ok(handle)
  }

  /// Add an element or an element tree to the UI
  ///
  /// Use the `rect` parameter to specify the position and size of the element\
  /// (usually, the size of the window/screen)
  ///
  /// ## Panics:
  /// If called while the UI is not active (call [`UiInstance::begin`] first)
  pub fn add(&mut self, element: impl UiElement, rect: impl Into<Rect>) {
    assert!(self.state, "must call UiInstance::begin before adding elements");
    let rect: Rect = rect.into();
    let layout = LayoutInfo {
      position: rect.position,
      max_size: rect.size,
      direction: Direction::Vertical,
      remaining_space: None,
    };
    let measure = element.measure(MeasureContext {
      state: &self.stateful_state,
      layout: &layout,
      painter: &self.painter,
    });
    element.process(ProcessContext {
      measure: &measure,
      state: &mut self.stateful_state,
      layout: &layout,
      painter: &mut self.painter,
      input: self.input.ctx(),
      signal: &mut self.signal,
    });
  }

  /// Prepare the UI for layout and processing\
  /// You must call this function at the beginning of the frame, before adding any elements\
  ///
  /// ## Panics:
  /// If called twice in a row (for example, if you forget to call [`UiInstance::end`])\
  /// This is an indication of a bug in your code and should be fixed.
  pub fn begin(&mut self) {
    //check and update current state
    assert!(!self.state, "must call UiInstance::end before calling UiInstance::begin again");
    self.state = true;

    //first, drain and process the event queue
    self.input.update_state(&mut self.events);

    //then, reset the (remaining) signals
    self.signal.clear();

    //then, reset the draw commands
    std::mem::swap(&mut self.prev_draw_commands, &mut self.draw_commands);
    self.draw_commands.commands.clear();
    // self.draw_call_modified = false;

    //reset atlas modification flag
    // self.atlas.reset_modified();
  }

  /// End the frame and prepare the UI for rendering\
  /// You must call this function at the end of the frame, before rendering the UI
  ///
  /// ## Panics:
  /// If called without calling [`UiInstance::begin`] first. (or if called twice)\
  /// This is an indication of a bug in your code and should be fixed.
  pub fn end(&mut self) {
    //check and update current state
    assert!(self.state, "must call UiInstance::begin before calling UiInstance::end");
    self.state = false;

    //check if the draw commands have been modified
    if self.draw_commands.commands == self.prev_draw_commands.commands {
      return
    }

    //if they have, rebuild the draw call and set the modified flag
    self.draw_call_id += 1;
  }

  /// Get the draw call information for the current frame
  ///
  /// This function should only be used by the render backend.\
  /// You should not call this directly unless you're implementing a custom render backend
  ///
  /// You should only call this function *after* [`UiInstance::end`]\
  /// Calling it in the middle of a frame will result in a warning but will not cause a panic\
  /// (please note that doing so is probably a mistake and should be fixed in your code)\
  /// Doing so anyway will return draw call data for the previous frame, but the `modified` flag will *always* be incorrect until [`UiInstance::end`] is called
  ///
  pub fn draw_call(&self) -> RenderInfo{
    if self.state {
      log::warn!("UiInstance::draw_call called while in the middle of a frame, this is probably a mistake");
    }
    RenderInfo {
      id: self.draw_call_id,
      list: &self.draw_commands,
    }
  }

  /// Get the texture atlas size and data for the current frame
  ///
  /// This function should only be used by the render backend.\
  /// You should not call this directly unless you're implementing a custom render backend
  ///
  /// You should only call this function *after* [`UiInstance::end`]\
  /// Calling it in the middle of a frame will result in a warning but will not cause a panic\
  /// (please note that doing so is probably a mistake and should be fixed in your code)\
  /// Using this function in the middle of a frame will return partially modified atlas data that may be outdated or incomplete\
  /// This will lead to rendering artifacts, 1-frame delays and flashes and is probably not what you want
  ///
  /// Make sure to check [`TextureAtlasMeta::modified`] to see if the texture has been modified
  /// since the beginning of the current frame before uploading it to the GPU
  pub fn atlas(&self) -> TextureAtlasMeta {
    if self.state {
      log::warn!("UiInstance::atlas called while in the middle of a frame, this is probably a mistake");
    }
    self.atlas.meta()
  }

  /// Push a platform event to the UI event queue
  ///
  /// You should call this function *before* calling [`UiInstance::begin`] or after calling [`UiInstance::end`]\
  /// Calling it in the middle of a frame will result in a warning but will not cause a panic\
  /// (please note that doing so is probably a mistake and should be fixed in your code)\
  /// In this case, the event will be processed in the next frame, but in some cases may affect the current frame.\
  /// (The exact behavior is not guaranteed and you should avoid doing this if possible)
  ///
  /// This function should only be used by the platform backend.\
  /// You should not call this directly unless you're implementing a custom platform backend
  /// or have a very specific usecase
  pub fn push_event(&mut self, event: UiEvent) {
    if self.state {
      log::warn!("UiInstance::push_event called while in the middle of a frame, this is probably a mistake");
    }
    self.events.push(event);
  }

  /// Push a "fake" signal to the UI signal queue
  pub fn push_signal<T: Signal + 'static>(&mut self, signal: T) {
    self.signal.add(signal);
  }

  //TODO: offer a non-consuming version of this function for T: Clone

  /// Process all signals of a given type
  ///
  /// This clears the signal queue for the given type and iterates over all signals
  pub fn process_signals<T: Signal + 'static>(&mut self, f: impl FnMut(T)) {
    self.signal.drain::<T>().for_each(f);
  }
}

impl Default for UiInstance {
  fn default() -> Self {
    Self::new()
  }
}
