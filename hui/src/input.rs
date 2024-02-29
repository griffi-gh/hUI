//! keyboard, mouse, and touch input handling

use std::hash::{Hash, Hasher};
use glam::Vec2;
use hashbrown::HashMap;
use nohash_hasher::{BuildNoHashHasher, NoHashHasher};
use tinyset::{Fits64, Set64, SetU32};
use crate::{event::{EventQueue, UiEvent}, rectangle::Rect};

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
#[repr(u8)]
pub enum KeyboardKey {
  //Keyboard buttons:
  A = 0, B = 1, C = 2, D = 3, E = 4, F = 5, G = 6, H = 7, I = 8, J = 9, K = 10, L = 11, M = 12, N = 13, O = 14, P = 15, Q = 16, R = 17, S = 18, T = 19, U = 20, V = 21, W = 22, X = 23, Y = 24, Z = 25,
  Num0 = 26, Num1 = 27, Num2 = 28, Num3 = 29, Num4 = 30, Num5 = 31, Num6 = 32, Num7 = 33, Num8 = 34, Num9 = 35,
  Np0 = 36, Np1 = 37, Np2 = 38, Np3 = 39, Np4 = 40, Np5 = 41, Np6 = 42, Np7 = 43, Np8 = 44, Np9 = 45,
  NpDivide = 46, NpMultiply = 47, NpSubtract = 48, NpAdd = 49, NpEnter = 50, NpDecimal = 51,
  F1 = 52, F2 = 53, F3 = 54, F4 = 55, F5 = 56, F6 = 57, F7 = 58, F8 = 59, F9 = 60, F10 = 61, F11 = 62, F12 = 63,
  Up = 64, Down = 65, Left = 66, Right = 67,
  Space = 68, Enter = 69, Escape = 70, Backspace = 71, Tab = 72, CapsLock = 73,
  LControl = 74, RControl = 75, LShift = 76, RShift = 77, LAlt = 78, RAlt = 79, LSuper = 80, RSuper = 81,
  Grave = 82, Minus = 83, Equals = 84, LeftBracket = 85, RightBracket = 86, Backslash = 87, Semicolon = 88, Apostrophe = 89, Comma = 90, Period = 91, Slash = 92,
  Insert = 93, Delete = 94, Home = 95, End = 96, PageUp = 97, PageDown = 98, PrintScreen = 99, ScrollLock = 100, Pause = 101, Menu = 102, NumLock = 103,
  //Multimedia keys and android-specific (e.g. volume keys):
  Mute = 104, VolumeUp = 105, VolumeDown = 106, MediaPlay = 107, MediaStop = 108, MediaNext = 109, MediaPrevious = 110,
  //Keycode:
  /// Represents a key code.
  ///
  /// This enum variant holds an unsigned 32-bit integer representing a key code.\
  /// The value of the key code is not standardized and may change depending on the platform or the backend used.
  KeyCode(u32),
}

macro_rules! impl_fits64_for_keyboard_key {
  ($($i:ident = $v:literal),*) => {
    impl Fits64 for KeyboardKey {
      unsafe fn from_u64(x: u64) -> Self {
        match x {
          $( $v => KeyboardKey::$i, )*
          _ => KeyboardKey::KeyCode(x as u32),
        }
      }

      fn to_u64(self) -> u64 {
        match self {
          $( KeyboardKey::$i => $v, )*
          KeyboardKey::KeyCode(x) => x as u64 | 0x8000000000000000u64,
        }
      }
    }
  };
}

impl_fits64_for_keyboard_key!(
  A = 0, B = 1, C = 2, D = 3, E = 4, F = 5, G = 6, H = 7, I = 8, J = 9, K = 10, L = 11, M = 12, N = 13, O = 14, P = 15, Q = 16, R = 17, S = 18, T = 19, U = 20, V = 21, W = 22, X = 23, Y = 24, Z = 25,
  Num0 = 26, Num1 = 27, Num2 = 28, Num3 = 29, Num4 = 30, Num5 = 31, Num6 = 32, Num7 = 33, Num8 = 34, Num9 = 35,
  Np0 = 36, Np1 = 37, Np2 = 38, Np3 = 39, Np4 = 40, Np5 = 41, Np6 = 42, Np7 = 43, Np8 = 44, Np9 = 45,
  NpDivide = 46, NpMultiply = 47, NpSubtract = 48, NpAdd = 49, NpEnter = 50, NpDecimal = 51,
  F1 = 52, F2 = 53, F3 = 54, F4 = 55, F5 = 56, F6 = 57, F7 = 58, F8 = 59, F9 = 60, F10 = 61, F11 = 62, F12 = 63,
  Up = 64, Down = 65, Left = 66, Right = 67,
  Space = 68, Enter = 69, Escape = 70, Backspace = 71, Tab = 72, CapsLock = 73,
  LControl = 74, RControl = 75, LShift = 76, RShift = 77, LAlt = 78, RAlt = 79, LSuper = 80, RSuper = 81,
  Grave = 82, Minus = 83, Equals = 84, LeftBracket = 85, RightBracket = 86, Backslash = 87, Semicolon = 88, Apostrophe = 89, Comma = 90, Period = 91, Slash = 92,
  Insert = 93, Delete = 94, Home = 95, End = 96, PageUp = 97, PageDown = 98, PrintScreen = 99, ScrollLock = 100, Pause = 101, Menu = 102, NumLock = 103,
  Mute = 104, VolumeUp = 105, VolumeDown = 106, MediaPlay = 107, MediaStop = 108, MediaNext = 109, MediaPrevious = 110
);

/// Information about the state of a mouse button
#[derive(Default, Clone, Copy, Debug)]
pub struct MouseButtonState {
  /// Whether the input is currently active (i.e. the button is currently held down)
  pub state: ButtonState,
  /// Position at which the input was initiated (last time it was pressed **down**)
  pub start_position: Option<Vec2>,
}

#[derive(Default)]
pub struct MousePointer {
  pub current_position: Vec2,
  pub buttons: HashMap<MouseButton, MouseButtonState, BuildNoHashHasher<u16>>,
}

pub struct TouchFinger {
  /// Unique identifier of the pointer (finger)
  pub id: u32,
  pub current_position: Vec2,
  pub start_position: Vec2,
}

pub type PointerId = u32;

/// Represents a pointer (mouse or touch)
pub enum Pointer {
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

impl MouseButtonState {
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
  pointers: &'a HashMap<PointerId, Pointer, BuildNoHashHasher<PointerId>>,
  /// Set of filtered pointer IDs
  filtered: SetU32
}

impl<'a> PointerQuery<'a> {
  fn new(pointers: &'a HashMap<PointerId, Pointer, BuildNoHashHasher<PointerId>>) -> Self {
    Self {
      pointers,
      filtered: pointers.keys().copied().collect(),
    }
  }

  /// Filter pointers that are *currently* located within the specified rectangle
  pub fn within_rect(&mut self, rect: Rect) -> &mut Self {
    for (&idx, pointer) in self.pointers {
      if rect.contains_point(pointer.current_position()) {
        self.filtered.insert(idx);
      }
    }
    self
  }

  /// Check if any pointers matched the filter
  pub fn any_matched(&self) -> bool {
    !self.filtered.is_empty()
  }

  pub fn finish(&self) -> Vec<&'a Pointer> {
    self.filtered.iter()
      .map(|id| self.pointers.get(&id).unwrap())
      .collect()
  }
}

const MOUSE_POINTER_ID: u32 = u32::MAX;

pub(crate) struct UiInputState {
  pointers: HashMap<u32, Pointer, BuildNoHashHasher<u32>>,
  keyboard_state: Set64<KeyboardKey>,
}

impl UiInputState {
  pub fn new() -> Self {
    Self {
      pointers: HashMap::default(),
      keyboard_state: Set64::new(),
    }
  }

  pub fn query_pointer(&self) -> PointerQuery {
    PointerQuery::new(&self.pointers)
  }

  /// Drain the event queue and update the internal input state
  pub fn update_state(&mut self, event_queue: &mut EventQueue) {
    for event in event_queue.drain() {
      match event {
        UiEvent::MouseMove(pos) => {
          let Pointer::MousePointer(mouse) = self.pointers.entry(MOUSE_POINTER_ID)
            .or_insert(Pointer::MousePointer(MousePointer::default())) else { unreachable!() };
          mouse.current_position = pos;
        },
        UiEvent::MouseButton { button, state } => {
          let Pointer::MousePointer(mouse) = self.pointers.entry(MOUSE_POINTER_ID)
            .or_insert(Pointer::MousePointer(MousePointer::default())) else { unreachable!() };
          let button_state = mouse.buttons.entry(button)
            .or_insert(MouseButtonState::default());
          button_state.state = state;
          button_state.start_position = state.is_pressed().then_some(mouse.current_position);
        },
        UiEvent::KeyboardButton { key, state } => {
          todo!()
        },
        _ => (), //TODO: Handle other events
      }
    }
  }
}
