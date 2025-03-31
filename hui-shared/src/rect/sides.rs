use core::ops::Add;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Product, Sub, SubAssign, Sum};

/// Represents 4 sides of a rectangular shape.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, Debug, Add, AddAssign, Sub, SubAssign, Neg, Mul, MulAssign, Div, DivAssign, Sum, Product)]
pub struct Sides<T> {
  pub top: T,
  pub bottom: T,
  pub left: T,
  pub right: T,
}

impl<T: Clone> Sides<T> {
  #[inline]
  pub fn all(value: T) -> Self {
    Self {
      top: value.clone(),
      bottom: value.clone(),
      left: value.clone(),
      right: value,
    }
  }

  #[inline]
  pub fn horizontal_vertical(horizontal: T, vertical: T) -> Self {
    Self {
      top: vertical.clone(),
      bottom: vertical,
      left: horizontal.clone(),
      right: horizontal,
    }
  }
}

impl<T: Add + Clone> Sides<T> {
  #[inline]
  pub fn sum_horizontal(&self) -> <T as Add>::Output {
    self.left.clone() + self.right.clone()
  }

  #[inline]
  pub fn sum_vertical(&self) -> <T as Add>::Output {
    self.top.clone() + self.bottom.clone()
  }
}

macro_rules! impl_sides_glam_fns {
  (
    $($numeric_type:ty => $glam_type:ty),*
    $(,)?
  ) => {
    $(
      impl Sides<$numeric_type> {
        pub fn top_left(&self) -> $glam_type {
          <$glam_type>::new(self.left, self.top)
        }

        pub fn top_right(&self) -> $glam_type {
          <$glam_type>::new(self.right, self.top)
        }

        pub fn bottom_left(&self) -> $glam_type {
          <$glam_type>::new(self.left, self.bottom)
        }

        pub fn bottom_right(&self) -> $glam_type {
          <$glam_type>::new(self.right, self.bottom)
        }

        pub fn sum_vec(&self) -> $glam_type {
          <$glam_type>::new(self.left + self.right, self.top + self.bottom)
        }
      }
    )*
  };
}

impl_sides_glam_fns!(
  f32 => glam::Vec2,
  f64 => glam::DVec2,

  u8 => glam::U8Vec2,
  u16 => glam::U16Vec2,
  u32 => glam::UVec2,
  u64 => glam::U64Vec2,

  i8 => glam::I8Vec2,
  i16 => glam::I16Vec2,
  i32 => glam::IVec2,
  i64 => glam::I64Vec2,
);


impl<T: Clone> From<T> for Sides<T> {
  fn from(value: T) -> Self {
    Self::all(value)
  }
}

impl<T: Clone> From<(T, T)> for Sides<T> {
  fn from((horizontal, vertical): (T, T)) -> Self {
    Self::horizontal_vertical(horizontal, vertical)
  }
}

impl<T> From<(T, T, T, T)> for Sides<T> {
  fn from((top, bottom, left, right): (T, T, T, T)) -> Self {
    Self { top, bottom, left, right }
  }
}
