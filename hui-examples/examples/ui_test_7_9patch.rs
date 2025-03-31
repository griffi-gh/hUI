use glam::vec2;
use hui::{
  color,
  element::{
    container::Container,
    frame_view::FrameView,
    slider::Slider,
    text::Text,
    UiElementExt
  },
  frame::nine_patch::{NinePatchAsset, NinePatchFrame},
  layout::Alignment,
  rect::Rect,
  signal::Signal,
  size,
};
use hui_painter::texture::SourceTextureFormat;

#[path = "../boilerplate.rs"]
#[macro_use]
mod boilerplate;

#[derive(Signal)]
struct SetValue(f32);

ui_main!(
  "hUI: 9-Patch demo",
  init: |ui| {
    (
      NinePatchAsset {
        // FIXME add image loader here
        image: {
          let data = std::fs::read("./hui-examples/assets/ninepatch_button.png").unwrap();
          let image = image::load_from_memory(&data[..]).unwrap();
          let width = image.width() as usize;
          let data = image.as_rgba8().unwrap().as_raw();
          ui.textures_mut().add_with_data(SourceTextureFormat::RGBA8, data, width)
        },
        size: (190, 49),
        scalable_region: Rect {
          position: vec2(8. / 190., 8. / 49.),
          size: vec2(1. - 16. / 190., 1. - 18. / 49.),
        },
      },
      0.33,
    )
  },
  run: |ui, size, (asset, value)| {
    Container::default()
      .with_size(size!(100%))
      .with_align(Alignment::Center)
      .with_gap(5.)
      .with_background(color::WHITE)
      .with_children(|ui| {
        Container::default()
          .with_size(size!(300, 100))
          .with_background(NinePatchFrame::from_asset(*asset).with_color(color::RED))
          .with_padding(10.)
          .with_children(|ui| {
            Text::new("Hello, world!\nThis is a 9-patch frame used as a background \nfor Container with a Text element.\nIt's scalable and looks great!\nBelow, there are two FillRects with the same \n9-patch frame used as the background.")
              .with_text_size(16.)
              .add_child(ui);
          })
          .add_child(ui);
        FrameView::default()
          .with_size(size!(600, 75))
          .with_frame(NinePatchFrame::from_asset(*asset).with_color(color::GREEN))
          .add_child(ui);
        Text::new("This one's fancy:")
          .with_color(color::BLACK)
          .with_text_size(32.)
          .add_child(ui);
        FrameView::default()
          .with_size(size!(700, 50))
          .with_frame(NinePatchFrame::from_asset(*asset).with_color((
            (1., 0., 1.),
            (0., 1., 1.),
            (1., 1., 0.),
            (0., 0., 1.),
          )))
          .add_child(ui);
        Text::new("Slider customized with `NinePatchFrame`s:")
          .with_color(color::BLACK)
          .with_text_size(32.)
          .add_child(ui);
        Slider::new(*value)
          .with_size(size!(50%, 30))
          .with_track_height(1.)
          .with_handle_size((20., 1.))
          .with_handle(NinePatchFrame::from_asset(*asset).with_color(color::CYAN))
          .with_track(NinePatchFrame::from_asset(*asset))
          .with_track_active(NinePatchFrame::from_asset(*asset).with_color(color::SKY_BLUE))
          .on_change(SetValue)
          .add_child(ui);
      })
      .add_root(ui, size);

    ui.process_signals::<SetValue>(|signal| *value = signal.0);
  }
);
