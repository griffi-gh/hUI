use std::time::Instant;
use glam::{UVec2, vec4};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  element::{
    container::Container, progress_bar::ProgressBar, rect::Rect, ElementList, UiElement
  }, layout::{Alignment, UiDirection, UiSize}, rectangle::{Corners, Sides}, UiInstance
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
          align: (Alignment::Begin, Alignment::Center).into(),
          size: (UiSize::Fraction(1.), UiSize::Fraction(1.)),
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
          align: (Alignment::End, Alignment::Center).into(),
          size: (UiSize::Fraction(1.), UiSize::Fraction(1.)),
          children: ElementList(vec![
            Box::new(ProgressBar {
              value: z,
              corner_radius: Corners::all(0.25 * ProgressBar::DEFAULT_HEIGHT),
              ..Default::default()
            }),
            Box::new(Container {
              size: (UiSize::Fraction(1.), UiSize::Auto),
              align: (Alignment::Center, Alignment::End).into(),
              padding: Sides::all(5.),
              gap: 10.,
              children: ElementList(vec![
                Box::new(Rect {
                  size: (UiSize::Fraction(0.5), UiSize::Static(30.)),
                  color: vec4(0.75, 0., 0., 1.).into()
                }),
                Box::new(Rect {
                  size: (UiSize::Fraction(z / 2. + 0.5), UiSize::Static(30.)),
                  color: Corners::left_right(
                    vec4(1., 0., 0., 1.),
                    vec4(0., 1., 0., 1.)
                  ).into(),
                }),
              ]),
              ..Default::default()
            }),
            Box::new(Rect {
              size: (UiSize::Fraction(z / 2. + 0.5), UiSize::Static(30.)),
              color: vec4(0., 0.75, 0., 1.).into()
            }),
            Box::new(Container {
              gap: 5.,
              padding: Sides::all(5.),
              background: vec4(0., 0., 0., 0.5).into(),
              direction: UiDirection::Horizontal,
              children: {
                let mut x: Vec<Box<dyn UiElement>> = vec![];
                for i in 0..10 {
                  x.push(Box::new(Rect {
                    size: (UiSize::Static(50.), UiSize::Static(50.)),
                    color: if i == 1 {
                      vec4(0.75, 0.75, 0.75, 0.75).into()
                    } else {
                      vec4(0.5, 0.5, 0.5, 0.75).into()
                    }
                  }));
                }
                ElementList(x)
              },
              ..Default::default()
            }),
            Box::new(Container {
              background: vec4(1., 0., 0., 1.).into(),
              padding: Sides {
                top: 10.,
                bottom: 20.,
                left: 30.,
                right: 40.,
              },
              corner_radius: Corners {
                top_left: 0.,
                top_right: 30.,
                bottom_left: 0.,
                bottom_right: 0.,
              },
              children: ElementList(vec![
                Box::new(Rect {
                  size: (UiSize::Static(50.), UiSize::Static(50.)),
                  color: vec4(1., 1., 1., 0.75).into()
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
