//! This module contains the definitions of the `Sides` and `Corners` structs,
//! which represent the sides and corners of a rectangular shape.

/// Represents 4 sides of a rectangular shape.
#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Corners<T> {
  pub top_left: T,
  pub top_right: T,
  pub bottom_left: T,
  pub bottom_right: T,
}

impl<T: Clone> Corners<T> {
  #[inline]
  pub fn all(value: T) -> Self {
    Self {
      top_left: value.clone(),
      top_right: value.clone(),
      bottom_left: value.clone(),
      bottom_right: value,
    }
  }

  #[inline]
  pub fn top_bottom(top: T, bottom: T) -> Self {
    Self {
      top_left: top.clone(),
      top_right: top,
      bottom_left: bottom.clone(),
      bottom_right: bottom,
    }
  }

  #[inline]
  pub fn left_right(left: T, right: T) -> Self {
    Self {
      top_left: left.clone(),
      top_right: right.clone(),
      bottom_left: left,
      bottom_right: right,
    }
  }
}

impl <T: Ord + Clone> Corners<T> {
  pub fn max(&self) -> T {
    self.top_left.clone()
      .max(self.top_right.clone())
      .max(self.bottom_left.clone())
      .max(self.bottom_right.clone())
      .clone()
  }
}

/// Represents 4 corners of a rectangular shape.
impl Corners<f32> {
  pub fn max_f32(&self) -> f32 {
    self.top_left
      .max(self.top_right)
      .max(self.bottom_left)
      .max(self.bottom_right)
  }
}

impl Corners<f64> {
  pub fn max_f64(&self) -> f64 {
    self.top_left
      .max(self.top_right)
      .max(self.bottom_left)
      .max(self.bottom_right)
  }
}

impl<T: Clone> From<T> for Corners<T> {
  fn from(value: T) -> Self {
    Self::all(value)
  }
}

impl<T> From<(T, T, T, T)> for Corners<T> {
  fn from((top_left, top_right, bottom_left, bottom_right): (T, T, T, T)) -> Self {
    Self {
      top_left,
      top_right,
      bottom_left,
      bottom_right,
    }
  }
}
