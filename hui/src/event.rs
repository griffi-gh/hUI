//! input, window events and event handling

use alloc::vec::{Vec, Drain};
use glam::Vec2;
use crate::input::{MouseButton, ButtonState, KeyboardKey};

#[derive(Clone, Copy, Debug)]
pub enum UiEvent {
  MouseMove(Vec2),
  MouseButton {
    button: MouseButton,
    state: ButtonState,
  },
  KeyboardButton {
    key: KeyboardKey,
    state: ButtonState,
  },
  TextInput(char),
}

#[derive(Default)]
pub(crate) struct EventQueue {
  events: Vec<UiEvent>,
}

impl EventQueue {
  pub(crate) fn new() -> Self {
    Self::default()
  }

  pub(crate) fn push(&mut self, event: UiEvent) {
    self.events.push(event);
  }

  pub(crate) fn drain(&mut self) -> Drain<UiEvent> {
    self.events.drain(..)
  }
}
