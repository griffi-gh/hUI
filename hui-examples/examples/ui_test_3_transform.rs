use std::f32::consts::PI;

use glam::{vec4, Vec2};
use hui::{
  element::{
    container::Container,
    text::Text,
    transformer::ElementTransformExt,
    UiElementExt
  }, frame::RectFrame, rect_frame, layout::Alignment, rect::Corners, size, text::FontHandle
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: Transform API demo",
  init: |ui| {
    let font = ui.add_font(include_bytes!("../assets/blink/Blink-ynYZ.otf"));
    ui.push_font(font);
    (std::time::Instant::now(),)
  },
  run: |ui, size, (instant,)| {
    let elapsed_sec = instant.elapsed().as_secs_f32();
    Container::default()
      .with_background(Corners {
        top_left: vec4(0.2, 0.2, 0.3, 1.),
        top_right: vec4(0.3, 0.3, 0.4, 1.),
        bottom_left: vec4(0.2, 0.3, 0.2, 1.),
        bottom_right: vec4(0.5, 0.4, 0.4, 1.),
      })
      .with_size(size!(100%))
      .with_align(Alignment::Center)
      .with_children(|ui| {
        Container::default()
          .with_align((Alignment::Center, Alignment::Begin))
          .with_padding(15.)
          .with_gap(10.)
          .with_background(rect_frame! {
            color: (0., 0., 0., 0.5),
            corner_radius: 8.
          })
          .with_children(|ui| {
            Text::default()
              .with_text("Did  you  know?")
              .with_text_size(18)
              .add_child(ui);
            Text::default()
              .with_text("You can die by jumping into the spike pit! :D\nCheck out the tutorial section for more tips.")
              .with_text_size(24)
              .with_font(FontHandle::default())
              .add_child(ui);
          })
          .transform()
          .scale(Vec2::splat(elapsed_sec.sin() * 0.1 + 1.))
          .rotate(elapsed_sec * PI / 4.)
          .add_child(ui);
      })
      .add_root(ui, size);
  }
);
