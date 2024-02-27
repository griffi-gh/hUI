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
pub enum UiSize {
  #[default]
  /// Automatically calculate size based on content
  Auto,
  /// Size as a ratio of parent size\
  /// Valid range: 0.0-1.0 (0-100%)
  ///
  /// Out of range values are allowed, but are not guaranteed to work as expected\
  /// (especially with negative values)
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

/// Constructs a `UiSize` or `UiSize2d` from a literal or expression
///
/// # Syntax:
/// - `auto` - `UiSize::Auto`
/// - `x` - `UiSize::Static(x)`
/// - `x%` - `UiSize::Fraction(x / 100.)` *(literal only)*
/// - `x/` - `UiSize::Fraction(x)`
///
/// ...where `x` is a literal, identifier or an expression wrapped in parentheses
///
/// # Note:
/// - If a single argument is provided, it creates a `UiSize` using the rules specified above\
/// - If two arguments are provided, it creates a `UiSize2d` with the first value as width and the second as height\
///   Example: `size!(100, 50%)` creates a `UiSize2d` with width `100` (`UiSize::Static(100.)`) and height `50%` (`UiSize::Fraction(0.5)`)
/// - `%` syntax is only valid for literals (`50%`), not expressions or identidiers.\
///   Use `/` instead (`(0.5 * x)/`, `x/`), but be aware of the different range (0.0-1.0) \
/// - Expressions must be wrapped in parentheses (for example: `(x + 5)`).\
///   This does not apply to single identifiers (`x`) or literals (`5`)
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
  ($x:literal /) => {
    $crate::layout::UiSize::Fraction($x as f32)
  };

  ($x:ident) => {
    $crate::layout::UiSize::Static($x as f32)
  };
  ($x:ident /) => {
    $crate::layout::UiSize::Fraction($x as f32)
  };

  (($x:expr)) => {
    $crate::layout::UiSize::Static(($x) as f32)
  };
  (($x:expr) /) => {
    $crate::layout::UiSize::Fraction(($x) as f32)
  };

  ($x:tt , $y:tt $($ys:tt)?) => {
    $crate::layout::UiSize2d {
      width: $crate::layout::size!($x),
      height: $crate::layout::size!($y $($ys)?),
    }
  };
  ($x:tt $($xs:tt)? , $y:tt $($ys:tt)?) => {
    $crate::layout::UiSize2d {
      width: $crate::layout::size!($x $($xs)?),
      height: $crate::layout::size!($y $($ys)?),
    }
  };
}
pub use size;

/// Represents the direction of the layout\
/// (for example, the direction of a container's children)\
///
/// - `Vertical` - Children are laid out from top to bottom\
/// - `Horizontal` - Children are laid out from left to right
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum UiDirection {
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
  pub direction: UiDirection,
}
