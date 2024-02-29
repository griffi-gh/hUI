//! keyboard, mouse, and touch input handling

use std::hash::{Hash, Hasher};
use glam::Vec2;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use tinyset::{SetU32, SetUsize};
use crate::rectangle::Rect;

/// Represents a mouse button.
///
/// Value of the `Other` variant is currently not standardized\
/// and may change depending on the platform or the backend used
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MouseButton {
  ///Primary mouse button (usually left)
  #[default]
  Primary,
  ///Secondary mouse button (usually right)
  Secondary,
  ///Middle mouse button (usually the wheel button)
  Middle,
  ///Other mouse button (e.g. extra buttons on a gaming mouse)
  ///
  ///Value is not standardized and may change depending on the platform or the backend used
  Other(u8),
}

// Manual hash impl only uses one hash call
impl Hash for MouseButton {
  fn hash<H: Hasher>(&self, state: &mut H) {
    match self {
      MouseButton::Primary => 0u16.hash(state),
      MouseButton::Secondary => 1u16.hash(state),
      MouseButton::Middle => 2u16.hash(state),
      MouseButton::Other(id) => ((*id as u16) << 8).hash(state),
    }
  }
}

/// Represents the state of a button, such as a mouse button or a keyboard key.\
/// Can be either `Pressed` (0) or `Released` (1).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonState {
  #[default]
  Released = 0,
  Pressed = 1,
}

impl ButtonState {
  pub fn is_pressed(self) -> bool {
    self == ButtonState::Pressed
  }
  pub fn is_released(self) -> bool {
    self == ButtonState::Released
  }
}

/// Represents a keyboard or other hardware key (for example volume buttons)
///
/// Values of the `KeyCode` variant are not standardized and may change depending on the platform or the backend used.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyboardKey {
  //Keyboard buttons:
  A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
  Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
  Np0, Np1, Np2, Np3, Np4, Np5, Np6, Np7, Np8, Np9,
  NpDivide, NpMultiply, NpSubtract, NpAdd, NpEnter, NpDecimal,
  F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
  Up, Down, Left, Right,
  Space, Enter, Escape, Backspace, Tab, CapsLock,
  LControl, RControl, LShift, RShift, LAlt, RAlt, LSuper, RSuper,
  Grave, Minus, Equals, LeftBracket, RightBracket, Backslash, Semicolon, Apostrophe, Comma, Period, Slash,
  Insert, Delete, Home, End, PageUp, PageDown, PrintScreen, ScrollLock, Pause, Menu, NumLock,
  //Multimedia keys and android-specific (e.g. volume keys):
  Mute, VolumeUp, VolumeDown, MediaPlay, MediaStop, MediaNext, MediaPrevious,
  //Keycode:
  /// Represents a key code.
  ///
  /// This enum variant holds an unsigned 32-bit integer representing a key code.
  /// The value of the key code is not standardized and may change depending on the platform or the backend used.
  KeyCode(u32),
}


/// Information about the state of a mouse button
pub(crate) struct ActiveMouseButton {
  /// Whether the input is currently active (i.e. the button is currently held down)
  pub active: bool,
  /// The button that initiated the input
  pub button: MouseButton,
  /// Position at which the input was initiated (last time it was pressed **down**)
  pub start_position: Option<Vec2>,
}

pub(crate) struct MousePointer {
  pub current_position: Vec2,
  pub buttons: HashMap<ButtonState, ActiveMouseButton, BuildNoHashHasher<u16>>,
}

pub(crate) struct TouchFinger {
  /// Unique identifier of the pointer (finger)
  pub id: u32,
  pub current_position: Vec2,
  pub start_position: Vec2,
}

/// Represents a pointer (mouse or touch)
pub(crate) enum Pointer {
  MousePointer(MousePointer),
  TouchFinger(TouchFinger),
}

impl Pointer {
  pub fn current_position(&self) -> Vec2 {
    match self {
      Pointer::MousePointer(mouse) => mouse.current_position,
      Pointer::TouchFinger(touch) => touch.current_position,
    }
  }
}

impl ActiveMouseButton {
  /// Check if the pointer (mouse or touch) was just pressed\
  /// (i.e. it was not pressed in the previous frame, but is pressed now)
  ///
  /// You should avoid using this, as it's not very intuitive for touch input (use `just_pressed` instead, if possible)
  pub fn just_pressed(&self) -> bool {
    todo!()
  }

  /// Check if the pointer (mouse or touch) was just released\
  /// (i.e. it was pressed in the previous frame, but is not pressed now)
  ///
  /// This is the preferred "on click" event for elements like buttons
  pub fn just_released(&self) -> bool {
    todo!()
  }
}

pub struct PointerQuery<'a> {
  pointers: &'a [Pointer],
  /// Set of pointer IDs to filter **out**
  filter_out: SetUsize,
}

impl<'a> PointerQuery<'a> {
  fn new(pointers: &'a [Pointer]) -> Self {
    Self {
      pointers,
      filter_out: SetUsize::new(),
    }
  }

  /// Filter pointers that are *currently* located within the specified rectangle
  pub fn within_rect(&mut self, rect: Rect) -> &mut Self {
    for (idx, pointer) in self.pointers.iter().enumerate() {
      if !rect.contains_point(pointer.current_position()) {
        self.filter_out.insert(idx);
      }
    }
    self
  }

  /// Check if any pointers matched the filter
  pub fn any_matched(&self) -> bool {
    self.filter_out.len() != self.pointers.len()
  }
}

pub(crate) struct UiInputState {
  pointers: Vec<Pointer>,
}

impl UiInputState {
  pub fn new() -> Self {
    Self {
      pointers: Vec::new(),
    }
  }

  pub fn query_pointer(&self) -> PointerQuery {
    PointerQuery::new(&self.pointers)
  }
}
