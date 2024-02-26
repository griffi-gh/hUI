use glam::UVec2;
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  UiInstance, color, size,
  layout::Alignment,
  rectangle::Corners,
  element::{
    container::Container, text::Text, UiElementExt
  },
};
use hui_glium::GliumUiRenderer;

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (_window, display) = SimpleWindowBuilder::new().build(&event_loop);

  let mut hui = UiInstance::new();
  let mut backend = GliumUiRenderer::new(&display);

  event_loop.run(|event, window_target| {
    window_target.set_control_flow(ControlFlow::Poll);
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        window_target.exit();
      },
      Event::AboutToWait => {
        let mut frame = display.draw();
        frame.clear_color_srgb(0.5, 0.5, 0.5, 0.);

        let resolution = UVec2::from(display.get_framebuffer_dimensions()).as_vec2();

        hui.begin();

        Container::default()
          .with_size(size!(100%, 50%))
          .with_align(Alignment::Center)
          .with_padding(5.)
          .with_corner_radius(10.)
          .with_background(color::RED)
          .with_children(|ui| {
            Text::default()
              .with_text("Hello, world")
              .with_text_size(100)
              .with_color(color::WHITE)
              .add_child(ui);
            Container::default()
              .with_padding((10., 20.))
              .with_corner_radius((10., 20., 50., 10.))
              .with_background(color::DARK_RED)
              .with_children(|ui| {
                Text::default()
                  .with_text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
                  .with_text_size(24)
                  .add_child(ui);
              })
              .add_child(ui);
          })
          .add_root(&mut hui, resolution);

        hui.end();

        backend.update(&hui);
        backend.draw(&mut frame, resolution);

        frame.finish().unwrap();
      }
      _ => (),
    }
  }).unwrap();
}
