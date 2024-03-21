
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
