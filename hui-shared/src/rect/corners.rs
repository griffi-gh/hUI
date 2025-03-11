/// Represents 4 corners of a rectangular shape.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Corners<T> {
  pub top_left: T,
  pub top_right: T,
  pub bottom_left: T,
  pub bottom_right: T,
}

impl<T> Corners<T> {
  #[inline]
  pub fn to_array(self) -> [T; 4] {
    [self.top_left, self.top_right, self.bottom_left, self.bottom_right]
  }

  #[inline]
  pub fn as_array(&self) -> [&T; 4] {
    [
      &self.top_left,
      &self.top_right,
      &self.bottom_left,
      &self.bottom_right,
    ]
  }

  pub fn as_array_mut(&mut self) -> [&mut T; 4] {
    [
      &mut self.top_left,
      &mut self.top_right,
      &mut self.bottom_left,
      &mut self.bottom_right,
    ]
  }
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

impl<T> IntoIterator for Corners<T> {
  type Item = T;
  type IntoIter = std::array::IntoIter<Self::Item, 4>;
  fn into_iter(self) -> Self::IntoIter {
    self.to_array().into_iter()
  }
}

impl<'a, T> IntoIterator for &'a Corners<T> {
  type Item = &'a T;
  type IntoIter = std::array::IntoIter<Self::Item, 4>;

  fn into_iter(self) -> Self::IntoIter {
    self.as_array().into_iter()
  }
}

impl<'a, T> IntoIterator for &'a mut Corners<T> {
  type Item = &'a mut T;
  type IntoIter = std::array::IntoIter<Self::Item, 4>;

  fn into_iter(self) -> Self::IntoIter {
    self.as_array_mut().into_iter()
  }
}

// over-engineered :p

// struct CornersIter<T> {
//   values: [ManuallyDrop<T>; 4],
//   curr: u8,
// }

// impl<T> Iterator for CornersIter<T> {
//   type Item = T;

//   fn next(&mut self) -> Option<Self::Item> {
//     if self.curr >= 4 {
//       return None
//     }
//     let result = unsafe {
//       ManuallyDrop::take(&mut self.values[self.curr as usize])
//     };
//     self.curr += 1;
//     Some(result)
//   }
// }

// impl<T> Drop for CornersIter<T> {
//   fn drop(&mut self) {
//     for i in self.curr..4 {
//       unsafe {
//         ManuallyDrop::drop(&mut self.values[i as usize]);
//       }
//     }
//   }
// }
