use glam::vec4;
use hui::{
  size, rect_frame,
  color,
  element::{
    container::Container,
    progress_bar::ProgressBar,
    text::Text,
    UiElementExt
  },
  layout::Alignment,
  rect::Corners,
  text::FontHandle
};

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

ui_main!(
  "hUI: Loading screen demo",
  init: |ui| {
    let font = ui.add_font(include_bytes!("../assets/blink/Blink-ynYZ.otf"));
    ui.push_font(font);
    (std::time::Instant::now(),)
  },
  run: |ui, size, (instant,)| {
    // Background color (gradient)
    Container::default()
      .with_size(size!(100%))
      .with_background(Corners {
        top_left: vec4(0.2, 0.2, 0.3, 1.),
        top_right: vec4(0.3, 0.3, 0.4, 1.),
        bottom_left: vec4(0.2, 0.3, 0.2, 1.),
        bottom_right: vec4(0.5, 0.4, 0.4, 1.),
      })
      .add_root(ui, size);

    // Loading text in the bottom right corner
    Container::default()
      .with_size(size!(100%))
      .with_align(Alignment::End)
      .with_padding(20.)
      .with_children(|ui| {
        Container::default()
          .with_padding((10., 15.))
          .with_background(rect_frame! {
            color: (0., 0., 0., 0.5),
            corner_radius: 8.,
          })
          .with_children(|ui| {
            let flash = 1. - 0.5 * (4. * instant.elapsed().as_secs_f32()).sin().powi(2);
            Text::default()
              .with_text("Loading...")
              .with_color((1., 1., 1., flash))
              .with_text_size(24)
              .add_child(ui);
          })
          .add_child(ui);
      })
      .add_root(ui, size);

    // Did you know? box in the center
    Container::default()
      .with_size(size!(100%))
      .with_align(Alignment::Center)
      .with_children(|ui| {
        Container::default()
          .with_align((Alignment::Center, Alignment::Begin))
          .with_padding(15.)
          .with_gap(10.)
          .with_background(rect_frame! {
            color: (0., 0., 0., 0.5),
            corner_radius: 8.,
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
          .add_child(ui);
      })
      .add_root(ui, size);

    // Progress bar at the bottom
    Container::default()
      .with_size(size!(100%))
      .with_align((Alignment::Center, Alignment::End))
      .with_children(|ui| {
        ProgressBar::default()
          .with_value((instant.elapsed().as_secs_f32() * 0.1) % 1.)
          .with_size(size!(100%, 5))
          .with_background((0., 0., 0., 0.5))
          .with_foreground(color::DARK_GREEN)
          .add_child(ui);
      })
      .add_root(ui, size);

    // Player XP and level (mock) in the top right corner
    Container::default()
      .with_size(size!(100%))
      .with_align((Alignment::End, Alignment::Begin))
      .with_padding(20.)
      .with_children(|ui| {
        Container::default()
          .with_padding(10.)
          .with_background(rect_frame!{
            color: (0., 0., 0., 0.5),
            corner_radius: 8.,
          })
          .with_children(|ui| {
            Text::default()
              .with_text("Level 5")
              .with_text_size(24)
              .add_child(ui);
            Text::default()
              .with_text("XP: 1234 / 5000")
              .with_text_size(18)
              .with_font(FontHandle::default())
              .add_child(ui);
          })
          .add_child(ui);
      })
      .add_root(ui, size);
  }
);
