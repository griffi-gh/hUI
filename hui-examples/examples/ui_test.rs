use hui::{
  color, size, rect_frame,
  element::{container::Container, text::Text, UiElementExt},
  frame::RectFrame,
  layout::Alignment,
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(|ui, size, _| {
  Container::default()
    .with_size(size!(100%, 50%))
    .with_align(Alignment::Center)
    .with_padding(5.)
    .with_gap(10.)
    .with_background(rect_frame! {
      color: (0.5, 0.5, 0.5, 1.),
      corner_radius: 10.,
    })
    .with_children(|ui| {
      Text::default()
        .with_text("Hello, world")
        .with_text_size(100)
        .with_color(color::BLACK)
        .add_child(ui);
      Container::default()
        .with_padding((10., 20.))
        .with_background(rect_frame! {
          color: color::DARK_RED,
          corner_radius: (2.5, 30., 2.5, 2.5),
        })
        .with_children(|ui| {
          Text::default()
            .with_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
            .with_text_size(24)
            .add_child(ui);
        })
        .add_child(ui);
    })
    .add_root(ui, size);
});
