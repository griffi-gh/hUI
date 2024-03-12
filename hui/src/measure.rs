//! element measurement, hints and responses

use glam::Vec2;
use crate::rectangle::Rect;

// #[non_exhaustive]
#[derive(Default)]
pub struct Hints {
  pub inner_content_size: Option<Vec2>,
  pub inner_content_size_cache: Option<Vec<Vec2>>,
}

#[derive(Default)]
pub struct Response {
  /// Computed size of the element
  pub size: Vec2,

  /// Hints for the layout system, can be used to optimize the layout engine.\
  /// These will never cause the UI to be rendered differently (assuming the values are correct)
  pub hints: Hints,

  /// Arbitrary user data, can be used to pass data (for example, cache) between measure and process stages
  pub user_data: Option<Box<dyn std::any::Any>>,

  /// If true, the element should always cause the content to wrap to the next line\
  /// (the element itself gets wrapped to the next line too)
  ///
  /// You should almost never set this, and the exact behavior may change in the future
  ///
  /// Currently, this forces wrapping even if Container::wrap is set to false
  pub should_wrap: bool,
}

impl Response {
  pub fn rect(&self, position: Vec2) -> Rect {
    Rect {
      position,
      size: self.size,
    }
  }
}
