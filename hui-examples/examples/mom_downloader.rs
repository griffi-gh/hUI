use std::time::Instant;
use hui::{
  element::{
    container::Container,
    progress_bar::ProgressBar,
    text::Text,
    UiElementExt,
  }, frame::FrameRect, layout::{Alignment, Direction}, size
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!{
  "Mom downloader 2000",
  init: |ui| {
    let font_handle = ui.add_font(include_bytes!("../assets/roboto/Roboto-Regular.ttf"));
    ui.push_font(font_handle);
    Instant::now()
  },
  run: |ui, max_size, instant| {
    let mom_ratio = (instant.elapsed().as_secs_f32() / 60.).powf(0.5);

    Container::default()
      .with_align(Alignment::Center)
      .with_size(size!(100%))
      .with_background((0.1, 0.1, 0.1))
      .with_children(|ui| {
        Container::default()
          .with_gap(5.)
          .with_padding(10.)
          .with_size(size!(450, auto))
          .with_background(
            FrameRect::color((0.2, 0.2, 0.5))
              .with_corner_radius(8.)
          )
          .with_children(|ui| {
            if instant.elapsed().as_secs_f32() < 5. {
              Text::default()
                .with_text("Downloading your mom...")
                .with_text_size(24)
                .add_child(ui);
              ProgressBar::default()
                .with_value(mom_ratio)
                .with_corner_radius(0.125 * ProgressBar::DEFAULT_HEIGHT)
                .add_child(ui);
              Container::default()
                .with_direction(Direction::Horizontal)
                .with_align((Alignment::End, Alignment::Center))
                .with_size(size!(100%, auto))
                .with_children(|ui| {
                  Text::default()
                    .with_text(format!("{:.2}% ({:.1} GB)", mom_ratio * 100., mom_ratio * 10000.))
                    .with_text_size(16)
                    .add_child(ui);
                })
                .add_child(ui);
            } else if instant.elapsed().as_secs() < 10 {
              Text::default()
                .with_text("Error 413: Request Entity Too Large")
                .with_color((1., 0.125, 0.125, 1.))
                .with_text_size(20)
                .add_child(ui);
              Text::default()
                .with_text(format!("Exiting in {}...", 10 - instant.elapsed().as_secs()))
                .with_text_size(16)
                .add_child(ui);
            } else {
              std::process::exit(0);
            }
          })
          .add_child(ui);
      })
      .add_root(ui, max_size)
  }
}
