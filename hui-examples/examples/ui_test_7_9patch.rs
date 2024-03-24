use std::time::Instant;
use hui::{
  color, element::{
    container::Container,
    fill_rect::FillRect,
    UiElementExt
  }, frame_rect, layout::{Alignment, Direction}, size
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: 9-Patch demo",
  init: |_| {

  },
  run: |ui, size, _| {
    Container::default()
      .with_size(size!(100%))
      .with_align(Alignment::Center)
      .with_background(color::WHITE)
      .with_children(|ui| {
        FillRect::default()
          .with_size(size!(300, 100))
          .with_frame(frame_rect! {
            color: color::RED
          })
          .add_child(ui);
      })
      .add_root(ui, size);
  }
);
