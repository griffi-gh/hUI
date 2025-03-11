use hui::{
  color, size,
  signal::Signal,
  layout::{Alignment, Direction},
  element::{
    container::Container,
    text::Text,
    image::Image,
    br::Break,
    interactable::ElementInteractableExt,
    UiElementExt,
  },
};
use hui_painter::texture::SourceTextureFormat;

#[derive(Signal)]
enum CounterSignal {
  Increment,
  Decrement,
}

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

const IMAGE_DATA: &[u8] = include_bytes!("../assets/icons/visual-studio-code-icon_32x32.rgba");

ui_main!(
  "hUI: Internal input test",
  init: |ui| {
    let image = ui.textures_mut().add_with_data(
      SourceTextureFormat::RGBA8,
      IMAGE_DATA, 32,
    );
    (0, image)
  },
  run: |ui, size, &mut (ref mut counter, image)| {
    Container::default()
      .with_size(size!(100%))
      .with_padding(10.)
      .with_align((Alignment::Center, Alignment::Begin))
      .with_direction(Direction::Horizontal)
      .with_gap(5.)
      .with_background((0.1, 0.1, 0.1))
      .with_wrap(true)
      .with_children(|ui| {
        Text::new("Number of images:")
          .with_text_size(24.)
          .add_child(ui);
        Break.add_child(ui);
        Container::default()
          .with_padding(10.)
          .with_background(color::ORANGE)
          .with_children(|ui| {
            Text::new("-")
              .with_text_size(32.)
              .add_child(ui);
          })
          .on_click(|| CounterSignal::Decrement)
          .add_child(ui);
        Container::default()
          .with_size(size!(60, auto))
          .with_align(Alignment::Center)
          .with_children(|ui| {
            Text::new(counter.to_string())
              .with_text_size(64.)
              .add_child(ui);
          })
          .add_child(ui);
        Container::default()
          .with_padding(10.)
          .with_background(color::ORANGE)
          .with_children(|ui| {
            Text::new("+")
              .with_text_size(32.)
              .add_child(ui);
          })
          .on_click(|| CounterSignal::Increment)
          .add_child(ui);
        Break.add_child(ui);
        for _ in 0..*counter {
          Image::new(image)
            .with_size(size!(48, 48))
            .add_child(ui);
        }
      })
      .add_root(ui, size);

    ui.process_signals(|sig| match sig {
      CounterSignal::Increment => *counter += 1,
      CounterSignal::Decrement => *counter -= 1,
    });
  }
);
