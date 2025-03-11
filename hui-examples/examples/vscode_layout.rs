//TODO: finish this demo

use hui::{
  color, size,
  layout::{Alignment, Direction},
  element::{
    container::Container,
    frame_view::FrameView,
    image::Image,
    text::Text,
    UiElementExt
  },
};
use hui_painter::texture::{SourceTextureFormat, TextureHandle};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

struct Stuff {
  vscode_icon: TextureHandle,
}

ui_main!(
  "hUI: vscode demo",
  init: |ui| {
    let handle = ui.add_font(include_bytes!("../assets/fira/FiraSans-Light.ttf"));
    ui.push_font_stack(handle);
    Stuff {
      vscode_icon: ui.add_image(SourceTextureFormat::RGBA8, include_bytes!("../assets/icons/visual-studio-code-icon_32x32.rgba"), 32),
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
          .with_padding(5.)
          .with_gap(15.)
          .with_background(color::rgb_hex(0x3d3c3e))
          .with_wrap(true) //XXX: not authentic but great for demostration
          .with_children(|ui| {
            Image::new(stuff.vscode_icon)
              .with_size(size!(auto, 24))
              .add_child(ui);
            for item in ["File", "Edit", "Selection", "View", "Go", "Run", "Terminal", "Help"] {
              Text::new(item)
                .with_text_size(15.)
                .add_child(ui);
            }
            Container::default()
              .with_size(size!(100%=, 100%))
              .with_align((Alignment::End, Alignment::Center))
              .with_children(|ui| {
                Text::new("- ×")
                  .with_text_size(32.)
                  .add_child(ui);
              })
              .add_child(ui);
          })
          .add_child(ui);
        FrameView::default()
          .with_size(size!(100%, 1))
          .with_frame(color::rgb_hex(0x2d2d30))
          .add_child(ui);
        Container::default()
          .with_size(size!(100%, 100%=))
          .with_direction(Direction::Horizontal)
          .with_children(|ui| {
            // Sidebar:
            Container::default()
              .with_size(size!(54, 100%))
              .with_background(color::rgb_hex(0x343334))
              .add_child(ui);
            FrameView::default()
              .with_size(size!(1, 100%))
              .with_frame(color::rgb_hex(0x2d2d30))
              .add_child(ui);

            // Explorer pane:
            Container::default()
              .with_size(size!(200, 100%))
              .with_padding((15., 8.))
              .with_background(color::rgb_hex(0x262526))
              .with_children(|ui| {
                Text::new("EXPLORER")
                  .add_child(ui);
              })
              .add_child(ui);

            // "Code" pane
            Container::default()
              .with_size(size!(100%=, 100%))
              .with_background(color::rgb_hex(0x1f1e1f))
              .add_child(ui);
          })
          .add_child(ui);

          //Status bar
          Container::default()
            .with_size(size!(100%, auto))
            .with_background(color::rgb_hex(0x0079cc))
            .with_direction(Direction::Horizontal)
            .with_gap(5.)
            .with_children(|ui| {
              Container::default()
                .with_background(color::rgb_hex(0x16815e))
                .with_padding((10., 2.))
                .with_children(|ui| {
                  Text::new("><")
                    .with_text_size(13.)
                    .add_child(ui);
                })
                .add_child(ui);
              Text::new("master")
                .with_text_size(15.)
                .add_child(ui);
            })
          .add_child(ui);
      })
      .add_root(ui, size);
  }
);
