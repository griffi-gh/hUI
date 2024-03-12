use glam::Vec2;
use crate::{
  draw::{
    ImageHandle, TextureFormat, UiDrawCall, UiDrawCommandList,
    atlas::{TextureAtlasManager, TextureAtlasMeta},
  },
  element::{MeasureContext, ProcessContext, UiElement},
  event::{EventQueue, UiEvent},
  input::UiInputState,
  layout::{Direction, LayoutInfo},
  signal::{SignalStore, UiSignal},
  state::StateRepo,
  text::{FontHandle, TextRenderer}
};

/// The main instance of the UI system.
///
/// In most cases, you should only have one instance of this struct, but multiple instances are allowed\
/// (Please note that it's possible to render multiple UI "roots" using a single instance)
pub struct UiInstance {
  //mouse_position: Vec2,
  stateful_state: StateRepo,
  //event_queue: VecDeque<UiEvent>,
  prev_draw_commands: UiDrawCommandList,
  draw_commands: UiDrawCommandList,
  draw_call: UiDrawCall,
  draw_call_modified: bool,
  text_renderer: TextRenderer,
  atlas: TextureAtlasManager,
  events: EventQueue,
  input: UiInputState,
  signal: SignalStore,
  //True if in the middle of a laying out a frame
  state: bool,
}

impl UiInstance {
  /// Crate and initialize a new instance of the UI
  ///
  /// In most cases, you should only do this *once*, during the initialization of your application
  pub fn new() -> Self {
    UiInstance {
      //mouse_position: Vec2::ZERO,
      stateful_state: StateRepo::default(),
      //event_queue: VecDeque::new(),
      // root_elements: Vec::new(),
      prev_draw_commands: UiDrawCommandList::default(),
      draw_commands: UiDrawCommandList::default(),
      draw_call: UiDrawCall::default(),
      draw_call_modified: false,
      // ftm: FontTextureManager::default(),
      text_renderer: TextRenderer::new(),
      atlas: {
        let mut atlas = TextureAtlasManager::default();
        atlas.add_dummy();
        atlas
      },
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
  pub fn add_font(&mut self, font: &[u8]) -> FontHandle {
    self.text_renderer.add_font_from_bytes(font)
  }

  /// Add an image to the texture atlas\
  /// Accepted texture formats are `Rgba` and `Grayscale`
  ///
  /// Returns an image handle ([`ImageHandle`])\
  /// This handle can be used to reference the texture in draw commands\
  /// It's a light reference and can be cloned/copied freely, but will not be cleaned up even when dropped
  pub fn add_image(&mut self, format: TextureFormat, data: &[u8], width: usize) -> ImageHandle {
    self.atlas.add(width, data, format)
  }

  /// Push a font to the font stack\
  /// The font will be used for all text rendering until it is popped
  ///
  /// This function is useful for replacing the default font, use sparingly\
  /// (This library attempts to be stateless, however passing the font to every text element is not very practical)
  pub fn push_font(&mut self, font: FontHandle) {
    self.text_renderer.push_font(font);
  }

  /// Pop a font from the font stack\
  ///
  /// ## Panics:
  /// If the font stack is empty
  pub fn pop_font(&mut self) {
    self.text_renderer.pop_font();
  }

  /// Get the current default font
  pub fn current_font(&self) -> FontHandle {
    self.text_renderer.current_font()
  }

  /// Add an element or an element tree to the UI
  ///
  /// Use the `max_size` parameter to specify the maximum size of the element\
  /// (usually, the size of the window/screen)
  ///
  /// ## Panics:
  /// If called while the UI is not active (call [`UiInstance::begin`] first)
  pub fn add<T: UiElement>(&mut self, element: T, max_size: Vec2) {
    assert!(self.state, "must call UiInstance::begin before adding elements");
    let layout = LayoutInfo {
      position: Vec2::ZERO,
      max_size,
      direction: Direction::Vertical,
    };
    let measure = element.measure(MeasureContext {
      state: &self.stateful_state,
      layout: &layout,
      text_measure: self.text_renderer.to_measure(),
      current_font: self.text_renderer.current_font(),
      images: self.atlas.context(),
    });
    element.process(ProcessContext {
      measure: &measure,
      state: &mut self.stateful_state,
      layout: &layout,
      draw: &mut self.draw_commands,
      text_measure: self.text_renderer.to_measure(),
      current_font: self.text_renderer.current_font(),
      images: self.atlas.context(),
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
    self.draw_call_modified = false;

    //reset atlas modification flag
    self.atlas.reset_modified();
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
    self.draw_call = UiDrawCall::build(&self.draw_commands, &mut self.atlas, &mut self.text_renderer);
    self.draw_call_modified = true;
  }

  /// Get the draw call information for the current frame
  ///
  /// This function should only be used by the render backend.\
  /// You should not call this directly unless you're implementing a custom render backend
  ///
  /// Returns a tuple with a boolean indicating if the buffers have been modified since the last frame
  ///
  /// You should only call this function *after* [`UiInstance::end`]\
  /// Calling it in the middle of a frame will result in a warning but will not cause a panic\
  /// (please note that doing so is probably a mistake and should be fixed in your code)\
  /// Doing so anyway will return draw call data for the previous frame, but the `modified` flag will *always* be incorrect until [`UiInstance::end`] is called
  ///
  pub fn draw_call(&self) -> (bool, &UiDrawCall) {
    if self.state {
      log::warn!("UiInstance::draw_call called while in the middle of a frame, this is probably a mistake");
    }
    (self.draw_call_modified, &self.draw_call)
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
  pub fn push_signal<T: UiSignal + 'static>(&mut self, signal: T) {
    self.signal.add(signal);
  }

  /// Process all signals of a given type
  ///
  /// This clears the signal queue for the given type and iterates over all signals
  pub fn process_signals<T: UiSignal + 'static>(&mut self, f: impl FnMut(T)) {
    self.signal.drain::<T>().for_each(f);
  }
}

impl Default for UiInstance {
  fn default() -> Self {
    Self::new()
  }
}
