//WARNING: THIS EXAMPLE IS EXTREMELY OUTDATED AND USES DEPRECATED API

use std::time::Instant;
use glam::{UVec2, vec4};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  element::{
    container::Container, fill_rect::FrameView, progress_bar::ProgressBar, ElementList, UiElement
  }, frame::FrameRect, layout::{Alignment, Direction, Size}, rect::{Corners, Sides}, UiInstance
};
use hui_glium::GliumUiRenderer;

fn main() {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (_window, display) = SimpleWindowBuilder::new().build(&event_loop);

  let mut hui = UiInstance::new();
  let mut backend = GliumUiRenderer::new(&display);

  let instant = Instant::now();
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

        let z = instant.elapsed().as_secs_f32().sin().powi(2);

        hui.add(Container {
          gap: 5.,
          padding: Sides::all(5.),
          align: (Alignment::Center, Alignment::Begin).into(),
          size: (Size::Relative(1.), Size::Relative(1.)).into(),
          children: ElementList(vec![
            Box::new(ProgressBar {
              value: 0.5,
              ..Default::default()
            }),
          ]),
          ..Default::default()
        }, resolution);

        hui.add(Container {
          gap: 5.,
          padding: Sides::all(5.),
          align: (Alignment::Center, Alignment::End).into(),
          size: (Size::Relative(1.), Size::Relative(1.)).into(),
          children: ElementList(vec![
            Box::new(ProgressBar {
              value: z,
              // corner_radius: Corners::all(0.25 * ProgressBar::DEFAULT_HEIGHT),
              ..Default::default()
            }),
            Box::new(Container {
              size: (Size::Relative(1.), Size::Auto).into(),
              align: (Alignment::End, Alignment::Center).into(),
              padding: Sides::all(5.),
              gap: 10.,
              children: ElementList(vec![
                Box::new(FrameView {
                  size: (Size::Relative(0.5), Size::Absolute(30.)).into(),
                  frame: Box::new(vec4(0.75, 0., 0., 1.)),
                }),
                Box::new(FrameView {
                  size: (Size::Relative(z / 2. + 0.5), Size::Absolute(30.)).into(),
                  frame: Box::new(Corners::left_right(
                    vec4(1., 0., 0., 1.),
                    vec4(0., 1., 0., 1.)
                  )),
                }),
              ]),
              ..Default::default()
            }),
            Box::new(FrameView {
              size: (Size::Relative(z / 2. + 0.5), Size::Absolute(30.)).into(),
              frame: Box::new(vec4(0., 0.75, 0., 1.)),
            }),
            Box::new(Container {
              gap: 5.,
              padding: Sides::all(5.),
              background_frame: Box::new(FrameRect::color(vec4(0., 0., 0., 0.5))),
              direction: Direction::Horizontal,
              children: {
                let mut x: Vec<Box<dyn UiElement>> = vec![];
                for i in 0..10 {
                  x.push(Box::new(FrameView {
                    size: (Size::Absolute(50.), Size::Absolute(50.)).into(),
                    frame: Box::new(if i == 1 {
                      vec4(0.75, 0.75, 0.75, 0.75)
                    } else {
                      vec4(0.5, 0.5, 0.5, 0.75)
                    }),
                  }));
                }
                ElementList(x)
              },
              ..Default::default()
            }),
            Box::new(Container {
              background_frame: Box::new(FrameRect::color((1., 0., 0.)).with_corner_radius(Corners {
                top_left: 0.,
                top_right: 30.,
                bottom_left: 0.,
                bottom_right: 0.,
              })),
              padding: Sides {
                top: 10.,
                bottom: 20.,
                left: 30.,
                right: 40.,
              },
              children: ElementList(vec![
                Box::new(FrameView {
                  size: (Size::Absolute(50.), Size::Absolute(50.)).into(),
                  frame: Box::new(vec4(1., 1., 1., 0.75)),
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
