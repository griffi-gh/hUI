use std::time::Instant;
use glam::{UVec2, vec4};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  element::{
    container::Container,
    progress_bar::ProgressBar,
    text::Text, ElementList,
  },
  layout::{Alignment, UiDirection, UiSize},
  rectangle::{Corners, Sides},
  UiInstance,
};
use hui_glium::GliumUiRenderer;

fn elements(mut f: impl FnMut(&mut Vec<Box<dyn hui::element::UiElement>>)) -> ElementList {
  let mut e = vec![];
  f(&mut e);
  ElementList(e)
}

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new().build(&event_loop);
  window.set_title("Mom Downloader 2000");

  let mut hui = UiInstance::new();
  let mut backend = GliumUiRenderer::new(&display);

  let font_handle = hui.add_font(include_bytes!("../assets/roboto/Roboto-Regular.ttf"));

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
          background: vec4(0.1, 0.1, 0.1, 1.).into(),
          children: ElementList(vec![Box::new(Container {
            gap: 5.,
            padding: Sides::all(10.),
            size: (UiSize::Static(450.), UiSize::Auto),
            background: vec4(0.2, 0.2, 0.5, 1.).into(),
            corner_radius: Corners::all(8.),
            children: elements(|el| {
              if instant.elapsed().as_secs_f32() < 5. {
                el.push(Box::new(Text {
                  text: "Downloading your mom...".into(),
                  font: font_handle,
                  text_size: 24,
                  ..Default::default()
                }));
                el.push(Box::new(ProgressBar {
                  value: mom_ratio,
                  corner_radius: Corners::all(0.125 * ProgressBar::DEFAULT_HEIGHT),
                  ..Default::default()
                }));
                el.push(Box::new(Container {
                  direction: UiDirection::Horizontal,
                  align: (Alignment::End, Alignment::Center).into(),
                  size: (UiSize::Fraction(1.), UiSize::Auto),
                  children: ElementList(vec![Box::new(Text {
                    text: format!("{:.2}% ({:.1} GB)", mom_ratio * 100., mom_ratio * 10000.).into(),
                    font: font_handle,
                    text_size: 16,
                    ..Default::default()
                  })]),
                  ..Default::default()
                }));
              } else if instant.elapsed().as_secs() < 10 {
                el.push(Box::new(Text {
                  text: "Error 413: Request Entity Too Large".into(),
                  font: font_handle,
                  color: vec4(1., 0.125, 0.125, 1.),
                  text_size: 20,
                  ..Default::default()
                }));
                el.push(Box::new(Text {
                  text: format!("Exiting in {}...", 10 - instant.elapsed().as_secs()).into(),
                  font: font_handle,
                  text_size: 16,
                  ..Default::default()
                }));
              } else {
                window_target.exit();
              }
            }),
            ..Default::default()
          })]),
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
