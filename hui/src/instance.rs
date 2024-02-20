use std::collections::VecDeque;
use glam::Vec2;
use crate:: {
  layout::{UiDirection, LayoutInfo},
  element::{MeasureContext, ProcessContext, UiElement},
  event::UiEvent,
  state::StateRepo,
  draw::{UiDrawCommandList, UiDrawPlan},
  text::{TextRenderer, FontTextureInfo, FontHandle},
};

/// The main instance of the UI system.\
/// In most cases, you should only have one instance of this struct.
pub struct UiInstance {
  //mouse_position: Vec2,
  stateful_state: StateRepo,
  //event_queue: VecDeque<UiEvent>,
  prev_draw_commands: UiDrawCommandList,
  draw_commands: UiDrawCommandList,
  draw_plan: UiDrawPlan,
  draw_plan_modified: bool,
  text_renderer: TextRenderer,
  events: VecDeque<UiEvent>,
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
      draw_plan: UiDrawPlan::default(),
      draw_plan_modified: false,
      // ftm: FontTextureManager::default(),
      text_renderer: TextRenderer::new(),
      events: VecDeque::new(),
    }
  }

  /// Parse and add a font from a raw byte slice to the UI\
  /// Returns a font handle.
  pub fn add_font_from_bytes(&mut self, font: &[u8]) -> FontHandle {
    self.text_renderer.add_font_from_bytes(font)
  }

  /// Add an element or an element tree to the UI
  ///
  /// Use the `max_size` parameter to specify the maximum size of the element\
  /// (usually, the size of the window/screen)
  pub fn add<T: UiElement>(&mut self, element: T, max_size: Vec2) {
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
    std::mem::swap(&mut self.prev_draw_commands, &mut self.draw_commands);
    self.draw_plan_modified = false;
    self.draw_commands.commands.clear();
    self.text_renderer.reset_frame();
  }

  /// End the frame and prepare the UI for rendering
  ///
  /// You must call this function at the end of the frame, before rendering the UI
  pub fn end(&mut self) {
    if self.draw_commands.commands == self.prev_draw_commands.commands {
      return
    }
    self.draw_plan = UiDrawPlan::build(&self.draw_commands, &mut self.text_renderer);
    self.draw_plan_modified = true;
  }

  /// Get the draw plan (a list of draw calls) for the current frame
  ///
  /// This function should only be used by the render backend.\
  /// You should not call this directly unless you're implementing a custom render backend
  ///
  /// Returns a tuple with a boolean indicating if the draw plan was modified since the last frame
  pub fn draw_plan(&self) -> (bool, &UiDrawPlan) {
    (self.draw_plan_modified, &self.draw_plan)
  }

  /// Get the font texture for the current frame
  ///
  /// This function should only be used by the render backend.\
  /// You should not call this directly unless you're implementing a custom render backend
  ///
  /// Make sure to check `FontTextureInfo::modified` to see if the texture was modified
  /// since the last frame before uploading it to the GPU
  pub fn font_texture(&self) -> FontTextureInfo {
    self.text_renderer.font_texture()
  }

  /// Push a platform event to the UI event queue
  ///
  /// This function should only be used by the platform backend.\
  /// You should not call this directly unless you're implementing a custom platform backend
  /// or have a very specific usecase
  pub fn push_event(&mut self, event: UiEvent) {
    self.events.push_back(event);
  }
}

impl Default for UiInstance {
  fn default() -> Self {
    Self::new()
  }
}
