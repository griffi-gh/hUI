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
