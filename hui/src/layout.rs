//! Layout related types and functions

use glam::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, PartialOrd, Ord)]
pub enum Alignment {
  #[default]
  Begin = 0,
  Center = 1,
  End = 2,
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
