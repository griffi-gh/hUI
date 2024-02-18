use std::time::Instant;
use glam::{UVec2, vec4};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  element::{
    container::{Alignment, Container, Sides}, progress_bar::ProgressBar, rect::Rect, text::Text, UiElement
  }, interaction::IntoInteractable, IfModified, UiDirection, UiInstance, UiSize
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

        hui.add(Container {
          gap: 5.,
          padding: Sides::all(5.),
          align: (Alignment::Center, Alignment::Center),
          size: (UiSize::Percentage(1.), UiSize::Percentage(1.)),
          elements: vec![
            Box::new(Text {
              text: "Hello, world!\nGoodbye, world!\nowo\nuwu".into(),
              text_size: 120,
              ..Default::default()
            }),
          ],
          ..Default::default()
        }, resolution);

        hui.end();

        backend.update(&hui);
        backend.draw(&mut frame, resolution);

        frame.finish().unwrap();
      }
      _ => (),
    }
  }).unwrap();
}
