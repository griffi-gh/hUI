//! Layout related types and functions

use glam::Vec2;

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

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum UiSize {
  #[default]
  /// Automatically calculate size based on content
  Auto,
  /// Size as a ratio of parent size\
  /// Valid range: 0.0-1.0 (0-100%)
  Fraction(f32),
  /// Static size in pixels
  Static(f32),
}

impl From<f32> for UiSize {
  #[inline]
  fn from(value: f32) -> Self {
    Self::Static(value)
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UiSize2d {
  pub width: UiSize,
  pub height: UiSize,
}

impl From<(UiSize, UiSize)> for UiSize2d {
  #[inline]
  fn from((width, height): (UiSize, UiSize)) -> Self {
    Self { width, height }
  }
}

//XXX: should this exist?
impl From<UiSize> for UiSize2d {
  #[inline]
  fn from(size: UiSize) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

//TODO?: add `UiSize2d` from `(Into<UiSize>, Into<UiSize>)` or Into<UiSize> conversion

/// Create a `UiSize` or `UiSize2d` from a literal
/// # Syntax:
/// - `auto` - `UiSize::Auto`
/// - `x` - `UiSize::Static(x)`
/// - `x%` - `UiSize::Fraction(x / 100.)`
///
/// If two values are provided, it creates a `UiSize2d` with the first value as width and the second as height
#[macro_export]
macro_rules! size {
  (auto) => {
    $crate::layout::UiSize::Auto
  };
  ($x:literal) => {
    $crate::layout::UiSize::Static($x as f32)
  };
  ($x:literal %) => {
    $crate::layout::UiSize::Fraction($x as f32 / 100.)
  };
  ($x:literal , $y:tt $($ys:tt)?) => {
    $crate::layout::UiSize2d {
      width: $crate::layout::size!($x),
      height: $crate::layout::size!($y $($ys)?),
    }
  };
  ($x:literal $($xs:tt)? , $y:tt $($ys:tt)?) => {
    $crate::layout::UiSize2d {
      width: $crate::layout::size!($x $($xs)?),
      height: $crate::layout::size!($y $($ys)?),
    }
  };
}
pub use size;

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
