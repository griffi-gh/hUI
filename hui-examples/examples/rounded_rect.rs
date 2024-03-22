//WARNING: THIS EXAMPLE IS EXTREMELY OUTDATED AND USES DEPRECATED API

use glam::{vec4, UVec2};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  element::{
    container::Container,
    text::Text, ElementList
  },
  layout::{Alignment, Direction, Size},
  rect::{Corners, Sides},
  UiInstance
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
          gap: 10.,
          align: Alignment::Center.into(),
          size: (Size::Relative(1.), Size::Relative(1.)).into(),
          children: ElementList(vec![
            Box::new(Container {
              align: Alignment::Center.into(),
              size: (Size::Relative(0.5), Size::Relative(0.5)).into(),
              background: vec4(1., 0., 0., 1.).into(),
              corner_radius: Corners {
                top_left: 10.,
                top_right: 20.,
                bottom_left: 50.,
                bottom_right: 80.
              },
              children: ElementList(vec![
                Box::new(Container {
                  padding: Sides::all(20.),
                  direction: Direction::Horizontal,
                  align: Alignment::Center.into(),
                  size: (Size::Auto, Size::Auto).into(),
                  background: vec4(0.1, 0.1, 0.1, 0.5).into(),
                  corner_radius: Corners::all(8.),
                  children: ElementList(vec![
                    Box::new(Text {
                      text: "Corners".into(),
                      text_size: 50,
                      color: vec4(1., 1., 1., 1.),
                      ..Default::default()
                    }),
                    Box::new(Text {
                      text: "!".into(),
                      text_size: 50,
                      color: vec4(1., 1., 0., 1.),
                      ..Default::default()
                    }),
                  ]),
                  ..Default::default()
                }),
              ]),
              ..Default::default()
            }),
            Box::new(Container {
              gap: 10.,
              direction: Direction::Horizontal,
              children: ElementList(vec![
                Box::new(Container {
                  size: (Size::Absolute(100.), Size::Absolute(100.)).into(),
                  background: Corners::left_right(
                    vec4(1., 0., 0., 1.),
                    vec4(0., 1., 0., 1.)
                  ).into(),
                  corner_radius: Corners::all(0.),
                  ..Default::default()
                }),
                Box::new(Container {
                  size: (Size::Absolute(100.), Size::Absolute(100.)).into(),
                  background: Corners::left_right(
                    vec4(1., 0., 0., 1.),
                    vec4(0., 1., 0., 1.)
                  ).into(),
                  corner_radius: Corners::all(10.),
                  ..Default::default()
                }),
                Box::new(Container {
                  size: (Size::Absolute(100.), Size::Absolute(100.)).into(),
                  background: Corners::left_right(
                    vec4(1., 0., 0., 1.),
                    vec4(0., 1., 0., 1.)
                  ).into(),
                  corner_radius: Corners::all(20.),
                  ..Default::default()
                }),
                Box::new(Container {
                  size: (Size::Absolute(100.), Size::Absolute(100.)).into(),
                  background: Corners::left_right(
                    vec4(1., 0., 0., 1.),
                    vec4(0., 1., 0., 1.)
                  ).into(),
                  corner_radius: Corners::all(30.),
                  ..Default::default()
                }),
              ]),
              ..Default::default()
            }),
          ]),
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
