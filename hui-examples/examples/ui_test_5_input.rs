use std::time::Instant;
use hui::{
  color, size,
  layout::{Alignment, Direction},
  element::{
    container::Container,
    fill_rect::FillRect,
    interactable::ElementInteractableExt,
    UiElementExt
  },
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: Internal input test",
  init: |_| {
    0
  },
  run: |ui, size, n| {
    Container::default()
      .with_size(size!(100%))
      .with_align(Alignment::Center)
      .with_background(color::WHITE)
      .with_children(|ui| {
        FillRect::default()
          .with_size(size!(40))
          .with_corner_radius(8.)
          .with_background(color::DARK_RED)
          .into_interactable()
          .on_click(|| {
            println!("clicked");
          })
          .add_child(ui);
      })
      .add_root(ui, size);
  }
);
