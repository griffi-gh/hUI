
/// Constructs a `Size` or `Size2d` from a literal or expression
///
/// # Syntax:
/// - `auto` - `Size::Auto`
/// - `x` - `Size::Absolute(x)`
/// - `x%` - `Size::Relative(x / 100.)` *(literal only)*
/// - `x/` - `Size::Relative(x)`
///
/// ...where `x` is a literal, identifier or an expression wrapped in parentheses
///
/// # Note:
/// - If a single argument is provided, it creates a `Size` using the rules specified above\
/// - If two arguments are provided, it creates a `Size2d` with the first value as width and the second as height\
///   Example: `size!(100, 50%)` creates a `Size2d` with width `100` (`Size::Absolute(100.)`) and height `50%` (`Size::Relative(0.5)`)
/// - `%` syntax is only valid for literals (`50%`), not expressions or identidiers.\
///   Use `/` instead (`(0.5 * x)/`, `x/`), but be aware of the different range (0.0-1.0) \
/// - Expressions must be wrapped in parentheses (for example: `(x + 5)`).\
///   This does not apply to single identifiers (`x`) or literals (`5`)
#[macro_export]
macro_rules! size {
  (auto) => {
    $crate::layout::Size::Auto
  };

  ($x:literal) => {
    $crate::layout::Size::Absolute($x as f32)
  };
  ($x:literal %) => {
    $crate::layout::Size::Relative($x as f32 / 100.)
  };
  ($x:literal /) => {
    $crate::layout::Size::Relative($x as f32)
  };

  ($x:ident) => {
    $crate::layout::Size::Absolute($x as f32)
  };
  ($x:ident /) => {
    $crate::layout::Size::Relative($x as f32)
  };

  (($x:expr)) => {
    $crate::layout::Size::Absolute(($x) as f32)
  };
  (($x:expr) /) => {
    $crate::layout::Size::Relative(($x) as f32)
  };

  ($x:tt , $y:tt $($ys:tt)?) => {
    $crate::layout::Size2d {
      width: $crate::size!($x),
      height: $crate::size!($y $($ys)?),
    }
  };
  ($x:tt $($xs:tt)? , $y:tt $($ys:tt)?) => {
    $crate::layout::Size2d {
      width: $crate::size!($x $($xs)?),
      height: $crate::size!($y $($ys)?),
    }
  };
}
