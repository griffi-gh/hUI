use std::time::Instant;
use hui::{
  color, element::{
    container::Container,
    fill_rect::FrameView,
    UiElementExt
  }, frame_rect, layout::{Alignment, Direction}, size
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: Wrapping demo",
  init: |_| {
    Instant::now()
  },
  run: |ui, size, instant| {
    let width_ratio = 0.5 + 0.5 * instant.elapsed().as_secs_f32().sin().powi(2);
    Container::default()
      .with_size(size!(width_ratio/, 100%))
      .with_direction(Direction::Horizontal)
      .with_align(Alignment::Center)
      .with_padding(5.)
      .with_gap(10.)
      .with_background(color::WHITE)
      .with_wrap(true)
      .with_children(|ui| {
        for i in 0..10 {
          FrameView::default()
            .with_size(size!((40 + i * 10)))
            .with_frame(frame_rect! {
              color: color::DARK_RED,
              corner_radius: 8.
            })
            .add_child(ui);
        }
      })
      .add_root(ui, size);
  }
);
