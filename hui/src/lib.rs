#![doc(html_logo_url = "https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/hui.svg")]
//!
//! Simple UI library for games and other interactive applications
//!
//! # Features
#![doc = document_features::document_features!()]

#![forbid(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]

use std::collections::VecDeque;

pub mod element;
pub mod event;
pub mod input;
pub mod draw;
pub mod measure;
pub mod state;
pub mod text;
pub mod interaction;

use element::{MeasureContext, ProcessContext, UiElement};
use event::UiEvent;
use state::StateRepo;
use draw::{UiDrawCommandList, UiDrawPlan};
use text::{TextRenderer, FontTextureInfo, FontHandle};
use glam::Vec2;

// pub struct ElementContext<'a> {
//   pub state: &'a mut StateRepo,
//   pub draw: &'a mut UiDrawCommands,
//   pub text: &'a mut TextRenderer,
// }
pub trait IfModified<T> {
  fn if_modified(&self) -> Option<&T>;
}

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

  pub fn add_font_from_bytes(&mut self, font: &[u8]) -> FontHandle {
    self.text_renderer.add_font_from_bytes(font)
  }

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

  pub fn begin(&mut self) {
    std::mem::swap(&mut self.prev_draw_commands, &mut self.draw_commands);
    self.draw_plan_modified = false;
    self.draw_commands.commands.clear();
    self.text_renderer.reset_frame();
  }

  pub fn end(&mut self) {
    if self.draw_commands.commands == self.prev_draw_commands.commands {
      return
    }
    self.draw_plan = UiDrawPlan::build(&self.draw_commands, &mut self.text_renderer);
    self.draw_plan_modified = true;
  }

  pub fn draw_plan(&self) -> (bool, &UiDrawPlan) {
    (self.draw_plan_modified, &self.draw_plan)
  }

  pub fn font_texture(&self) -> FontTextureInfo {
    self.text_renderer.font_texture()
  }

  pub fn push_event(&mut self, event: UiEvent) {
    self.events.push_back(event);
  }
}

impl Default for UiInstance {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum UiSize {
  #[default]
  Auto,
  Fraction(f32),
  Static(f32),
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiDirection {
  #[default]
  Vertical,
  Horizontal,
}

pub struct LayoutInfo {
  ///Not availabe during measuring step
  pub position: Vec2,
  pub max_size: Vec2,
  pub direction: UiDirection,
}

pub struct ElementList(Vec<Box<dyn UiElement>>);

impl ElementList {
  pub fn add(&mut self, element: impl UiElement + 'static) {
    self.0.push(Box::new(element));
  }
}
pub fn elements(f: impl FnOnce(&mut ElementList)) -> Vec<Box<dyn UiElement>> {
  let mut elements = ElementList(Vec::new());
  f(&mut elements);
  elements.0
}
