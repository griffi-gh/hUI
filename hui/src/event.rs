//! input, window events and event handling

use glam::Vec2;

/// Represents a mouse button.
///
/// Value of the `Other` variant is currently not standardized\
/// and may change depending on the platform or the backend used
#[derive(Clone, Copy, Debug, Default)]
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
