use hui_painter::{
  backend::BackendData, paint::command::{PaintCommand, PaintList}, presentation::Presentatation, text::FontHandle, texture::{SourceTextureFormat, TextureHandle}, PainterInstance
};
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  event::{EventQueue, UiEvent},
  font::FontStack,
  input::UiInputState,
  layout::{Direction, LayoutInfo},
  rect::Rect,
  signal::{Signal, SignalStore},
  state::StateRepo,
};

/// The main instance of the UI system.
///
/// In most cases, you should only have one instance of this struct, but multiple instances are allowed\
/// (Please note that it's possible to render multiple UI "roots" using a single instance)
pub struct UiInstance {
  // TODO Do not own Presentation/Painter
  painter: PainterInstance,
  presentation: Presentatation,
  paint_commands: PaintList,
  stateful_state: StateRepo,
  events: EventQueue,
  input: UiInputState,
  signal: SignalStore,
  font_stack: FontStack,

  /// Set to true if present has been called since the last begin_frame
  frame_presented: bool,
}

impl UiInstance {
  /// Crate and initialize a new instance of the UI
  ///
  /// In most cases, you should only do this *once*, during the initialization of your application
  pub fn new() -> Self {
    UiInstance {
      painter: PainterInstance::new(),
      presentation: Presentatation::new(),
      paint_commands: PaintList::default(),
      font_stack: FontStack::new(),
      stateful_state: StateRepo::new(),
      events: EventQueue::new(),
      input: UiInputState::new(),
      signal: SignalStore::new(),
      frame_presented: false,
    }
  }

  /// Parse and add a font from a raw byte slice to the UI\
  /// TrueType (`.ttf`/`.ttc`) and OpenType (`.otf`) fonts are supported\
  ///
  /// Returns a font handle ([`FontHandle`]).
  ///
  /// ## Panics:
  /// If the font data is invalid or corrupt
  #[deprecated(note = "use painter.fonts_mut().add instead")]
  pub fn add_font(&mut self, font: &[u8]) -> FontHandle {
    self.painter.fonts_mut().add(font)
  }

  /// Add an image to the texture atlas\
  /// Accepted texture formats are `Rgba` and `Grayscale`
  ///
  /// Returns an image handle ([`ImageHandle`])\
  /// This handle can be used to reference the texture in draw commands\
  /// It's a light reference and can be cloned/copied freely, but will not be cleaned up even when dropped
  #[deprecated(note = "use painter.textures_mut().atlas_mut().allocate_with_data instead")]
  pub fn add_image(&mut self, format: SourceTextureFormat, data: &[u8], width: usize) -> TextureHandle {
    // self.atlas().add(width, data, format)
    self.painter.textures_mut().allocate_with_data(format, data, width)
  }

  //TODO better error handling

  /// Add an image from a file to the texture atlas\
  /// (experimental, may be removed in the future)
  ///
  /// Requires the `image` feature
  ///
  /// # Panics:
  /// - If the file exists but contains invalid image data\
  ///   (this will change to a soft error in the future)
  ///
  /// Deprecated.
  #[cfg(feature = "image")]
  #[deprecated]
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
    let handle = self.add_image(
      SourceTextureFormat::RGBA8,
      image_rgba,
      image.width() as usize
    );

    Ok(handle)
  }

  /// Push a font to the font stack\
  /// The font will be used for all text rendering until it is popped
  ///
  /// This function is useful for replacing the default font, use sparingly\
  /// (This library attempts to be stateless, however passing the font to every text element is not very practical)
  pub fn push_font(&mut self, font: FontHandle) {
    self.font_stack.push(font);
  }

  /// Pop a font from the font stack\
  ///
  /// ## Panics:
  /// If the font stack is empty
  pub fn pop_font(&mut self) {
    self.font_stack.pop();
  }

  /// Get the current default font
  pub fn current_font(&self) -> Option<FontHandle> {
    self.font_stack.current()
  }

  /// Add an element or an element tree to the UI
  ///
  /// Use the `rect` parameter to specify the position and size of the element\
  /// (usually, the size of the window/screen)
  ///
  /// ## Panics:
  /// If called while the UI is not active (call [`UiInstance::begin`] first)
  pub fn add(&mut self, element: impl UiElement, rect: impl Into<Rect>) {
    let rect: Rect = rect.into();
    let layout = LayoutInfo {
      position: rect.position,
      max_size: rect.size,
      direction: Direction::Vertical,
      remaining_space: None,
    };
    // TODO handle font_stack.current() font being None
    let current_font = self.font_stack.current().expect("No current font");
    let measure = element.measure(MeasureContext {
      painter: &self.painter,
      state: &self.stateful_state,
      layout: &layout,
      current_font,
    });
    element.process(ProcessContext {
      painter: &mut self.painter,
      measure: &measure,
      state: &mut self.stateful_state,
      layout: &layout,
      paint_target: &mut self.paint_commands,
      input: self.input.ctx(),
      signal: &mut self.signal,
      current_font,
    });
  }

  /// Reset the state from the previous frame, and prepare the UI for layout and processing
  ///
  /// - You must call this function at the start of the frame, before adding any elements
  /// - Make sure to provide all of the events that happened since the last frame before calling this function, to avoid a 1-frame delay in event processing
  pub fn begin_frame(&mut self) {
    //first, drain and process the event queue
    self.input.update_state(&mut self.events);

    //then, reset the (remaining) signals
    self.signal.clear();

    // Clear the draw commands
    self.paint_commands.clear();
  }

  /// End rendering the current frame and present it
  ///
  /// You must call this function sometime at the end of the frame, after adding all elements but before rendering, but before running the render backend
  pub fn end_frame(&mut self) {
    self.presentation.draw(&mut self.painter, &self.paint_commands);
  }

  pub fn backend_data(&self) -> BackendData {
    self.painter.backend_data(&self.presentation)
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
  /// or have a very specific usecase (not using one)
  pub fn push_event(&mut self, event: UiEvent) {
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

  /// Get the paint commands needed to render the UI
  pub fn paint_command(&self) -> &impl PaintCommand {
    &self.paint_commands
  }
}

impl Default for UiInstance {
  fn default() -> Self {
    Self::new()
  }
}
