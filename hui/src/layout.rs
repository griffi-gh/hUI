//! element layout, alignment and sizing

use glam::{vec2, Vec2};

/// Controls wrapping behavior of elements
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Default)]
pub enum WrapBehavior {
  /// No wrapping is allowed, even if explicit line breaks is requested by the element
  Disable = 0,

  /// Allow wrapping if the element explicitly requests it (default behavior)
  #[default]
  Allow = 1,

  /// Elements will be wrapped automatically when they reach the maximum width/height of the container
  Enable = 2,
}

impl From<bool> for WrapBehavior {
  #[inline]
  fn from(value: bool) -> Self {
    match value {
      true => Self::Enable,
      false => Self::Disable,
    }
  }
}

impl WrapBehavior {
  /// Check if wrapping is allowed for the element
  #[inline]
  pub fn is_allowed(&self) -> bool {
    *self != Self::Disable
  }

  /// Check if wrapping is enabled for the element
  ///
  /// (Wrapping will be done automatically when the element reaches the maximum width/height)
  #[inline]
  pub fn is_enabled(&self) -> bool {
    *self == Self::Enable
  }
}

/// Alignment along a single axis
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
pub enum Alignment {
  /// Put the element at the beginning of the axis\
  /// (left for horizontal, top for vertical alignment)
  #[default]
  Begin = 0,

  /// Put the element in the center
  Center = 1,

  /// Put the element at the end of the axis\
  /// (right for horizontal, bottom for vertical alignment)
  End = 2,
}

/// Represents alignment in 2D space
///
/// - `horizontal` - alignment *along* x-axis (horizontal)\
/// - `vertical` - alignment *along* y-axis (vertical)
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
pub struct Alignment2d {
  /// Alignment *along* horizontal axis (X)
  ///
  /// ```text
  /// ├───────[  ]──────┤
  ///  ↑↑      ↑↑     ↑↑
  /// Begin  Center  End
  /// ```
  pub horizontal: Alignment,

  /// Alignment *along* vertical axis (Y)
  ///
  /// ```text
  ///   ┬   ←─ Begin
  ///   │
  /// [   ] ←─ Center
  ///   │
  ///   ┴   ←─ End
  /// ```
  pub vertical: Alignment,
}

impl Alignment2d {
  /// Create a new `Alignment2d` with the same alignment for both axes
  #[inline]
  pub const fn all(alignment: Alignment) -> Self {
    Self {
      horizontal: alignment,
      vertical: alignment,
    }
  }
}

impl From<(Alignment, Alignment)> for Alignment2d {
  #[inline]
  fn from((horizontal, vertical): (Alignment, Alignment)) -> Self {
    Self { horizontal, vertical }
  }
}

impl From<[Alignment; 2]> for Alignment2d {
  #[inline]
  fn from([horizontal, vertical]: [Alignment; 2]) -> Self {
    Self { horizontal, vertical }
  }
}

impl From<Alignment> for Alignment2d {
  #[inline]
  fn from(alignment: Alignment) -> Self {
    Self::all(alignment)
  }
}

/// Represents a single size dimension of an UI element.\
/// Can be either a static size in pixels, a fraction the parent size or auto-calculated\
/// (Meaning of `auto` is entirely dependent on the element).
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum Size {
  /// Automatically calculate size based on content
  #[default]
  Auto,

  /// Static size in pixels
  Absolute(f32),

  /// Size as a ratio of parent element size
  ///
  /// Expected range: `0.0..=1.0`
  Relative(f32),

  //TODO Remaining(f32)
}

impl From<f32> for Size {
  #[inline]
  fn from(value: f32) -> Self {
    Self::Absolute(value)
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Size2d {
  pub width: Size,
  pub height: Size,
}

impl From<(Size, Size)> for Size2d {
  #[inline]
  fn from((width, height): (Size, Size)) -> Self {
    Self { width, height }
  }
}

//XXX: should this exist?
impl From<Size> for Size2d {
  #[inline]
  fn from(size: Size) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

/// Represents the direction of the layout\
/// (for example, the direction of a container's children)\
///
/// - `Vertical` - Children are laid out from top to bottom\
/// - `Horizontal` - Children are laid out from left to right
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
  /// Children are laid out from top to bottom
  #[default]
  Vertical,
  /// Children are laid out from left to right
  Horizontal,
}

/// Represents the layout information required to measure, layout and render an element.\
/// Includes the position, maximum size, direction of the layout and other information
pub struct LayoutInfo {
  /// Screen-space coordinates of the top-left corner of the element.\
  /// Use this value during the layout step to render the element
  ///
  /// Not available during the measure step (will be set to zero)
  pub position: Vec2,

  /// Maximum size the element is allowed to take up
  pub max_size: Vec2,

  /// Current direction of the layout\
  /// (Usually matches direction of the parent container)
  pub direction: Direction,
}

/// Helper function to calculate the size of an element based on its layout and size information\
/// Used to help reduce code duplication in the `measure` method of UI elements
pub fn compute_size(layout: &LayoutInfo, size: Size2d, comfy_size: Vec2) -> Vec2 {
  let width = match size.width {
    Size::Auto => comfy_size.x,
    Size::Relative(fraction) => layout.max_size.x * fraction,
    Size::Absolute(size) => size,
  };
  let height = match size.height {
    Size::Auto => comfy_size.y,
    Size::Relative(fraction) => layout.max_size.y * fraction,
    Size::Absolute(size) => size,
  };
  vec2(width, height)
}
