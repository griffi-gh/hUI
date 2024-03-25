//! keyboard, mouse, and touch input handling

use std::hash::{Hash, Hasher};
use glam::Vec2;
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;
use tinyset::{Fits64, Set64};
use crate::{event::{EventQueue, UiEvent}, rect::Rect};

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

impl From<bool> for ButtonState {
  fn from(b: bool) -> Self {
    if b { ButtonState::Pressed } else { ButtonState::Released }
  }
}

impl From<ButtonState> for bool {
  fn from(s: ButtonState) -> Self {
    s.is_pressed()
  }
}

impl ButtonState {
  /// Returns `true` if the button is pressed
  pub fn is_pressed(self) -> bool {
    self == ButtonState::Pressed
  }

  /// Returns `true` if the button is released
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
      // SAFETY: not actually doing anything unsafe
      #[allow(unsafe_code)]
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

/// Information about the current state of a pressed mouse button
#[derive(Default, Clone, Copy, Debug)]
pub struct MouseButtonMeta {
  /// Position at which the input was initiated (last time it was pressed **down**)
  pub start_position: Vec2,
}

#[derive(Default)]
pub struct MouseState {
  /// Current position of the mouse pointer
  pub current_position: Vec2,

  /// Position of the mouse pointer on the previous frame
  pub prev_position: Vec2,

  /// Current state of each mouse button (if down)
  pub buttons: HashMap<MouseButton, MouseButtonMeta, BuildNoHashHasher<u16>>,

  /// mouse buttons that were released *in the current frame*
  pub released_buttons: HashMap<MouseButton, MouseButtonMeta, BuildNoHashHasher<u16>>,
}

/// Unique identifier of a touch pointer (finger)
pub type TouchId = u32;

pub struct TouchFinger {
  /// Unique identifier of the pointer/finger
  pub id: TouchId,
  /// Current position of the pointer/finger
  pub current_position: Vec2,
  pub start_position: Vec2,
}

pub(crate) struct UiInputState {
  // pointers: HashMap<u32, Pointer, BuildNoHashHasher<u32>>,
  mouse_pointer: MouseState,
  keyboard_state: Set64<KeyboardKey>,
  /// events that happened in the current frame
  just_happened: Vec<UiEvent>,
}

impl UiInputState {
  pub fn new() -> Self {
    Self {
      // pointers: HashMap::default(),
      mouse_pointer: MouseState::default(),
      keyboard_state: Set64::new(),
      just_happened: Vec::new(),
    }
  }

  /// Drain the event queue and update the internal input state
  ///
  /// This function should be called exactly once per frame
  pub fn update_state(&mut self, event_queue: &mut EventQueue) {
    self.mouse_pointer.prev_position = self.mouse_pointer.current_position;
    self.mouse_pointer.released_buttons.clear();
    self.just_happened.clear();
    self.just_happened.extend(event_queue.drain());
    for event in &self.just_happened {
      #[allow(clippy::single_match)]
      match event {
        UiEvent::MouseMove(pos) => {
          self.mouse_pointer.current_position = *pos;
        },
        UiEvent::MouseButton { button, state } => {
          match state {
            //wtf should we do with buttons that are pressed and released in the same frame?
            //i have no fvcking idea
            ButtonState::Pressed => {
              let button = self.mouse_pointer.buttons.entry(*button)
                .or_insert(MouseButtonMeta::default());
              button.start_position = self.mouse_pointer.current_position;
            },
            ButtonState::Released => {
              //log::trace!("Button {:?} was released", button);
              if let Some(button_meta) = self.mouse_pointer.buttons.remove(button) {
                //log::trace!("start pos was: {:?} current pos is: {:?}", button_meta.start_position, self.mouse_pointer.current_position);
                self.mouse_pointer.released_buttons.insert(*button, button_meta);
              } else {
                //huh
                //this can happen i guess ¯\_(=^･ω･^)_/¯
                self.mouse_pointer.released_buttons.insert(*button, MouseButtonMeta {
                  start_position: self.mouse_pointer.current_position,
                });
              }
            },
          }
        },
        UiEvent::KeyboardButton { key, state } => {
          match state {
            ButtonState::Pressed => self.keyboard_state.insert(*key),
            ButtonState::Released => self.keyboard_state.remove(key),
          };
        },
        //TODO touch, text input
        _ => (),
      }
    }
  }

  pub fn ctx(&self) -> InputCtx {
    InputCtx(self)
  }
}

/// Response for checks that involve an active pointer
#[derive(Clone, Copy, Debug)]
pub struct ActiveCheckResponse {
  /// Current position of the pointer inside the target rectangle's coordinate space
  pub position_in_rect: Vec2,

  /// Position of the pointer at the time the start of the input inside the target rectangle's coordinate space
  pub start_position_in_rect: Vec2,

  /// Position of the pointer on the previous frame inside the target rectangle's coordinate space
  pub last_position_in_rect: Vec2,
}

#[derive(Clone, Copy)]
pub struct InputCtx<'a>(&'a UiInputState);

impl<'a> InputCtx<'a> {
  /// Get the current position of the mouse pointer
  ///
  /// Do not use this function to check for hover, use [`InputCtx::check_hover`] instead
  pub fn mouse_position(&self) -> Vec2 {
    self.0.mouse_pointer.current_position
  }

  /// Get the current position of the mouse pointer within a rectangle
  ///
  /// Do not use this function to check for hover, use [`InputCtx::check_hover`] instead
  pub fn mouse_position_in_rect(&self, rect: Rect) -> Option<Vec2> {
    let pos = self.0.mouse_pointer.current_position;
    rect.contains_point(pos).then_some(pos - rect.position)
  }

  /// Get the state of a mouse button
  pub fn mouse_button_down(&self, button: MouseButton) -> ButtonState {
    self.0.mouse_pointer.buttons.contains_key(&button).into()
  }

  /// Get the start position of a mouse button\
  /// (Position at the last time it was pressed **down**)
  ///
  /// Returns `None` if the button is not currently down
  pub fn mouse_button_start_position(&self, button: MouseButton) -> Option<Vec2> {
    self.0.mouse_pointer.buttons.get(&button).map(|meta| meta.start_position)
  }

  /// Get the relative movement of the mouse pointer since the button was pressed down
  ///
  /// This function is similar to [`InputCtx::mouse_button_start_position`], but returns the relative movement instead of the absolute position
  pub fn mouse_button_relative_movement(&self, button: MouseButton) -> Option<Vec2> {
    let start = self.mouse_button_start_position(button)?;
    Some(self.mouse_position() - start)
  }

  /// Check if a rect can be considered "hovered"
  ///
  /// This can be triggered by multiple input sources, such as mouse, touch, etc.
  pub fn check_hover(&self, rect: Rect) -> bool {
    rect.contains_point(self.0.mouse_pointer.current_position)
  }

  /// Check if a rect can be considered "clicked" in the current frame
  ///
  /// This can be triggered by multiple input sources, such as mouse, touch, etc.\
  /// In case of a mouse, these conditions must be met:
  /// - The mouse button got released in the current frame
  /// - The mouse pointer is currently inside the rectangle
  /// - The mouse pointer was inside the rectangle at the time the button was pressed down
  ///
  /// By default, this function only checks for the primary mouse button\
  /// This is a limitation of the current API and may change in the future\
  /// (as the current implementation of this function checks for both mouse and touch input, and the touch input quite obviously only supports one "button")
  pub fn check_click(&self, rect: Rect) -> Option<ActiveCheckResponse> {
    let pos = self.0.mouse_pointer.current_position;
    self.0.mouse_pointer.released_buttons.get(&MouseButton::Primary).filter(|meta| {
      rect.contains_point(meta.start_position) && rect.contains_point(pos)
    }).map(|mi| ActiveCheckResponse {
      position_in_rect: pos - rect.position,
      start_position_in_rect: mi.start_position - rect.position,
      last_position_in_rect: self.0.mouse_pointer.prev_position - rect.position,
    })
  }

  // TODO: write better docs

  /// Check if a rect is being actively being interacted with (e.g. dragged)
  pub fn check_active(&self, rect: Rect) -> Option<ActiveCheckResponse> {
    self.0.mouse_pointer.buttons.get(&MouseButton::Primary).filter(|mi| {
      rect.contains_point(mi.start_position)
    }).map(|mi| ActiveCheckResponse {
      position_in_rect: self.0.mouse_pointer.current_position - rect.position,
      start_position_in_rect: mi.start_position - rect.position,
      last_position_in_rect: self.0.mouse_pointer.prev_position - rect.position,
    })
  }
}
