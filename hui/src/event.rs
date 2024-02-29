//! input, window events and event handling

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
