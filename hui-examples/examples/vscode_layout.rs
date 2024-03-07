//TODO: finish this demo

use std::time::Instant;
use hui::{
  color, size,
  layout::{Alignment, Direction},
  element::{
    container::Container,
    fill_rect::FillRect,
    UiElementExt
  },
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: vscode demo",
  init: |_| {
    Instant::now()
  },
  run: |ui, size, instant| {
    Container::default()
      .with_size(size!(100%))
      .with_background(color::WHITE)
      .add_root(ui, size);
  }
);
