use hui::{
  color, size,
  draw::TextureFormat,
  signal::UiSignal,
  layout::{Alignment, Direction},
  element::{
    container::Container,
    text::Text,
    image::Image,
    br::Br,
    interactable::ElementInteractableExt,
    slider::Slider,
    UiElementExt,
  },
};

enum CounterSignal {
  ChangeValue(u32)
}
impl UiSignal for CounterSignal {}

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

const IMAGE_DATA: &[u8] = include_bytes!("../assets/icons/visual-studio-code-icon_32x32.rgba");

ui_main!(
  "hUI: Internal input test",
  init: |ui| {
    let image = ui.add_image(TextureFormat::Rgba, IMAGE_DATA, 32);
    (0, image)
  },
  run: |ui, size, (ref mut counter, image)| {
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
          .with_text_size(24)
          .add_child(ui);
        Br.add_child(ui);
        Slider::new(*counter as f32 / 100.)
          .with_size(size!(66%, 20))
          .on_change(|x| {
            CounterSignal::ChangeValue((x * 100.).round() as u32)
          })
          .add_child(ui);
        Br.add_child(ui);
        for _ in 0..*counter {
          Image::new(*image)
            .with_size(size!(48, 48))
            .add_child(ui);
        }
      })
      .add_root(ui, size);

    ui.process_signals(|sig| match sig {
      CounterSignal::ChangeValue(v) => *counter = v,
    });
  }
);
