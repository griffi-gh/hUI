use hui::{
  color, size,
  layout::{Alignment, Direction},
  element::{
    container::Container,
    text::Text,
    interactable::ElementInteractableExt,
    UiElementExt
  },
  signal::UiSignal,
};

enum CounterSignal {
  Increment,
  Decrement,
}
impl UiSignal for CounterSignal {}

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: Internal input test",
  init: |_| {
    0
  },
  run: |ui, size, counter| {
    Container::default()
      .with_size(size!(100%))
      .with_align(Alignment::Center)
      .with_direction(Direction::Horizontal)
      .with_gap(5.)
      .with_background(color::WHITE)
      .with_children(|ui| {
        Container::default()
          .with_padding(10.)
          .with_corner_radius(8.)
          .with_background(color::DARK_RED)
          .with_children(|ui| {
            Text::new("-")
              .add_child(ui);
          })
          .on_click(CounterSignal::Decrement)
          .add_child(ui);
        Container::default()
          .with_size(size!(20, auto))
          .with_align(Alignment::Center)
          .with_children(|ui| {
            Text::new(counter.to_string())
              .with_color(color::BLACK)
              .with_text_size(32)
              .add_child(ui);
          })
          .add_child(ui);
        Container::default()
          .with_padding(10.)
          .with_corner_radius(8.)
          .with_background(color::DARK_RED)
          .with_children(|ui| {
            Text::new("+")
              .add_child(ui);
          })
          .on_click(CounterSignal::Increment)
          .add_child(ui);
      })
      .add_root(ui, size);

    ui.process_signals(|sig| {
      match sig {
        CounterSignal::Increment => *counter += 1,
        CounterSignal::Decrement => *counter -= 1,
      }
    });
  }
);
