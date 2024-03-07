//TODO: finish this demo

use hui::{
  color, size,
  draw::{ImageHandle, TextureFormat},
  layout::{Alignment, Direction},
  element::{
    container::Container,
    fill_rect::FillRect,
    image::Image,
    spacer::Spacer,
    text::Text,
    UiElementExt
  },
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

struct Stuff {
  vscode_icon: ImageHandle,
}

ui_main!(
  "hUI: vscode demo",
  init: |ui| {
    let handle = ui.add_font(include_bytes!("../assets/fira/FiraSans-Light.ttf"));
    ui.push_font(handle);
    Stuff {
      vscode_icon: ui.add_image(TextureFormat::Rgba, include_bytes!("../assets/icons/visual-studio-code-icon_32x32.rgba"), 32),
    }
  },
  run: |ui, size, stuff| {
    Container::default()
      .with_size(size!(100%))
      .with_children(|ui| {
        Container::default()
          .with_size(size!(100%, auto))
          .with_direction(Direction::Horizontal)
          .with_align((Alignment::Begin, Alignment::Center))
          .with_padding((5., 8.))
          .with_gap(15.)
          .with_background(color::rgb_hex(0x3d3c3e))
          .with_wrap(true) //XXX: not authentic but great for demostration
          .with_children(|ui| {
            Image::new(stuff.vscode_icon)
              .with_size(size!(auto, 24))
              .add_child(ui);
            Spacer(1.).add_child(ui);
            for item in ["File", "Edit", "Selection", "View", "Go", "Run", "Terminal", "Help"] {
              Text::new(item)
                .with_text_size(15)
                .add_child(ui);
            }
          })
          .add_child(ui);
        FillRect::default()
          .with_size(size!(100%, 1))
          .with_background(color::rgb_hex(0x2d2d30))
          .add_child(ui);
        Container::default()
          .with_size(size!(100%, 100%))
          .with_direction(Direction::Horizontal)
          .with_children(|ui| {
            Container::default()
              .with_size(size!(48, 100%))
              .with_background(color::rgb_hex(0x343334))
              .add_child(ui);
            FillRect::default()
              .with_size(size!(1, 100%))
              .with_background(color::rgb_hex(0x2d2d30))
              .add_child(ui);
            Container::default()
              .with_size(size!(200, 100%))
              .with_padding((15., 8.))
              .with_background(color::rgb_hex(0x1e1e1e))
              .with_children(|ui| {
                Text::new("EXPLORER")
                  .add_child(ui);
              })
              .add_child(ui);
          })
          .add_child(ui);

      })
      .add_root(ui, size);
  }
);
