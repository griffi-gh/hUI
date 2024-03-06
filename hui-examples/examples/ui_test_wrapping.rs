use hui::{
  color, size,
  layout::UiDirection,
  element::{
    container::Container,
    fill_rect::FillRect,
    text::Text,
    UiElementExt
  },
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(|ui, size, _| {
  Container::default()
    .with_size(size!(100%))
    .with_direction(UiDirection::Horizontal)
    .with_padding(5.)
    .with_gap(10.)
    .with_background(color::WHITE)
    .with_wrap(true)
    .with_children(|ui| {
      Text::default()
        .with_color(color::BLACK)
        .with_text("wrapping is not actually implemented yet")
        .add_child(ui);
      for _ in 0..10 {
        FillRect::default()
          .with_size(size!(100))
          .with_corner_radius(8.)
          .with_background(color::DARK_RED)
          .add_child(ui);
      }
    })
    .add_root(ui, size);
});
