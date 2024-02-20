use std::time::Instant;
use glam::{UVec2, vec4};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  UiInstance, elements,
  layout::{Alignment, UiDirection, UiSize},
  rectangle::{Corners, Sides},
  element::{
    container::Container,
    progress_bar::ProgressBar,
    text::Text,
  },
};
use hui_glium::GliumUiRenderer;

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
  window.set_title("Mom Downloader 2000");

  let mut hui = UiInstance::new();
  let mut backend = GliumUiRenderer::new(&display);

  let font_handle = hui.add_font_from_bytes(include_bytes!("../assets/roboto/Roboto-Regular.ttf"));

  let instant = Instant::now();

  event_loop.run(|event, window_target| {
    window_target.set_control_flow(ControlFlow::Poll);
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        window_target.exit();
      },
      Event::AboutToWait => {
        let mut frame = display.draw();
        frame.clear_color_srgb(0., 0., 0., 1.);

        let resolution = UVec2::from(display.get_framebuffer_dimensions()).as_vec2();

        hui.begin();

        let mom_ratio = (instant.elapsed().as_secs_f32() / 60.).powf(0.5);

        hui.add(Container {
          align: Alignment::Center.into(),
          size: (UiSize::Fraction(1.), UiSize::Fraction(1.)),
          background: Some(vec4(0.1, 0.1, 0.1, 1.)),
          elements: vec![Box::new(Container {
            gap: 5.,
            padding: Sides::all(10.),
            size: (UiSize::Static(450.), UiSize::Auto),
            background: Some(vec4(0.2, 0.2, 0.5, 1.)),
            corner_radius: Corners::all(8.),
            elements: elements(|el| {
              if instant.elapsed().as_secs_f32() < 5. {
                el.add(Text {
                  text: "Downloading your mom...".into(),
                  font: font_handle,
                  text_size: 24,
                  ..Default::default()
                });
                el.add(ProgressBar {
                  value: mom_ratio,
                  ..Default::default()
                });
                el.add(Container {
                  direction: UiDirection::Horizontal,
                  align: (Alignment::End, Alignment::Center).into(),
                  size: (UiSize::Fraction(1.), UiSize::Auto),
                  elements: vec![Box::new(Text {
                    text: format!("{:.2}% ({:.1} GB)", mom_ratio * 100., mom_ratio * 10000.).into(),
                    font: font_handle,
                    text_size: 16,
                    ..Default::default()
                  })],
                  ..Default::default()
                });
              } else if instant.elapsed().as_secs() < 10 {
                el.add(Text {
                  text: "Error 413: Request Entity Too Large".into(),
                  font: font_handle,
                  color: vec4(1., 0.125, 0.125, 1.),
                  text_size: 20,
                  ..Default::default()
                });
                el.add(Text {
                  text: format!("Exiting in {}...", 10 - instant.elapsed().as_secs()).into(),
                  font: font_handle,
                  text_size: 16,
                  ..Default::default()
                });
              } else {
                window_target.exit();
              }
            }),
            ..Default::default()
          })],
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
