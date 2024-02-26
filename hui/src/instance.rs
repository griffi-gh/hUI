use std::collections::VecDeque;
use glam::Vec2;
use crate::{
  draw::{
    atlas::{TextureAtlasManager, TextureAtlasMeta},
    UiDrawCall, UiDrawCommandList,
  },
  element::{MeasureContext, ProcessContext, UiElement},
  event::UiEvent,
  layout::{LayoutInfo, UiDirection},
  state::StateRepo,
  text::{FontHandle, TextRenderer}
};

/// The main instance of the UI system.\
/// In most cases, you should only have one instance of this struct.
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
  events: VecDeque<UiEvent>,
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
        //HACK: Ensure that vec(0, 0) uv is white square
        atlas.add_grayscale(1, &[255]);
        atlas
      },
      events: VecDeque::new(),
      state: false,
    }
  }

  /// Parse and add a font from a raw byte slice to the UI\
  /// Returns a font handle.
  pub fn add_font(&mut self, font: &[u8]) -> FontHandle {
    self.text_renderer.add_font_from_bytes(font)
  }

  /// Add an element or an element tree to the UI
  ///
  /// Use the `max_size` parameter to specify the maximum size of the element\
  /// (usually, the size of the window/screen)
  pub fn add<T: UiElement>(&mut self, element: T, max_size: Vec2) {
    assert!(self.state, "must call UiInstance::begin before adding elements");
    let layout = LayoutInfo {
      position: Vec2::ZERO,
      max_size,
      direction: UiDirection::Vertical,
    };
    let measure = element.measure(MeasureContext {
      state: &self.stateful_state,
      layout: &layout,
      text_measure: self.text_renderer.to_measure(),
    });
    element.process(ProcessContext {
      measure: &measure,
      state: &mut self.stateful_state,
      layout: &layout,
      draw: &mut self.draw_commands,
      text_measure: self.text_renderer.to_measure(),
    });
  }

  /// Prepare the UI for layout and processing
  ///
  /// You must call this function at the beginning of the frame, before adding any elements
  pub fn begin(&mut self) {
    assert!(!self.state, "must call UiInstance::end before calling UiInstance::begin again");
    self.state = true;
    std::mem::swap(&mut self.prev_draw_commands, &mut self.draw_commands);
    self.draw_call_modified = false;
    self.draw_commands.commands.clear();
    self.atlas.reset_modified();
  }

  /// End the frame and prepare the UI for rendering
  ///
  /// You must call this function at the end of the frame, before rendering the UI
  pub fn end(&mut self) {
    assert!(self.state, "must call UiInstance::begin before calling UiInstance::end");
    self.state = false;
    if self.draw_commands.commands == self.prev_draw_commands.commands {
      return
    }
    self.draw_call = UiDrawCall::build(&self.draw_commands, &mut self.atlas, &mut self.text_renderer);
    self.draw_call_modified = true;
  }

  /// Get the draw call information for the current frame
  ///
  /// This function should only be used by the render backend.\
  /// You should not call this directly unless you're implementing a custom render backend
  ///
  /// Returns a tuple with a boolean indicating if the buffers have been modified since the last frame
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
  /// This function should only be used by the platform backend.\
  /// You should not call this directly unless you're implementing a custom platform backend
  /// or have a very specific usecase
  pub fn push_event(&mut self, event: UiEvent) {
    if self.state {
      log::warn!("UiInstance::push_event called while in the middle of a frame, this is probably a mistake");
    }
    self.events.push_back(event);
  }
}

impl Default for UiInstance {
  fn default() -> Self {
    Self::new()
  }
}
