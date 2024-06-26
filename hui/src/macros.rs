
/// Constructs a `Size` or `Size2d` from a literal or expression
///
/// # Syntax:
/// - `auto` - `Size::Auto`
/// - `x` - `Size::Absolute(x)`
/// - `x%` - `Size::Relative(x / 100.)` *(literal only)*
/// - `x/` - `Size::Relative(x)`
/// - `x%=` - `Size::Remaining(x / 100.)` *(literal only)*
/// - `x/=` - `Size::Remaining(x)`
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
  ($x:literal %=) => {
    $crate::layout::Size::Remaining($x as f32 / 100.)
  };
  ($x:literal /=) => {
    $crate::layout::Size::Remaining($x as f32)
  };

  ($x:ident) => {
    $crate::layout::Size::Absolute($x as f32)
  };
  ($x:ident /) => {
    $crate::layout::Size::Relative($x as f32)
  };
  ($x:ident /=) => {
    $crate::layout::Size::Remaining($x as f32)
  };

  (($x:expr)) => {
    $crate::layout::Size::Absolute(($x) as f32)
  };
  (($x:expr) /) => {
    $crate::layout::Size::Relative(($x) as f32)
  };
  (($x:expr) /=) => {
    $crate::layout::Size::Remaining(($x) as f32)
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

/// Helper macro for constructing a `RectFrame`
///
/// # Example:
/// ```
/// _frame! {
///   color: (0.2, 0.2, 0.3, 1.),
///   corner_radius: 5.,
/// };
/// ```
///
/// # Note:
/// - If the `image` field is set, but not `color`, the `color` field will default to [`WHITE`](crate::color::WHITE) (to ensure visibility)
/// - If both `color` and `image` are not set, the `color` field will default to [`TRANSPARENT`](crate::color::TRANSPARENT)
#[macro_export]
macro_rules! rect_frame {
  {} => {
    $crate::frame::RectFrame::default()
  };

  // () => {
  //   $crate::frame::RectFrame::default()
  // };

  ($expr:expr) => {
    {
      let __frame: $crate::frame::RectFrame = $crate::frame::RectFrame::from($expr);
      __frame
    }
  };

  ($image:expr, $color:expr) => {
    $crate::frame::RectFrame::color_image($color, $image)
  };

  {$($ident:ident : $expr:expr),+$(,)?} => {
    {
      // ensure all identifiers are unique
      #[allow(non_upper_case_globals)]
      {$(const $ident: () = ();)+}

      // construct the RectFrame
      {
        let mut _frame = $crate::frame::RectFrame::default();
        let mut _color_is_set = false;
        let mut _image_is_set = false;
        $(
          {
            _frame.$ident = ($expr).into();
            _image_is_set |= stringify!($ident) == "image";
            _color_is_set |= stringify!($ident) == "color";
          }
        )+
        // set color to white if image is explicitly set to Some(...) but color is left as the default
        if _frame.image.is_some() && _image_is_set && !_color_is_set {
          _frame.color = (1., 1., 1., 1.).into();
        }
        _frame
      }
    }
  };
}
