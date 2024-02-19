use glam::{vec4, UVec2};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::{
  UiDirection, UiInstance, UiSize,
  element::{
    container::{Alignment, Container, Sides},
    text::Text
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

        hui.add(Container {
          align: (Alignment::Center, Alignment::Center),
          size: (UiSize::Fraction(1.), UiSize::Fraction(1.)),
          elements: vec![Box::new(Container {
            align: (Alignment::Center, Alignment::Center),
            size: (UiSize::Fraction(0.5), UiSize::Fraction(0.5)),
            background: Some(vec4(1., 0., 0., 1.)),
            corner_radius: Some(30.),
            elements: vec![
              Box::new(Container {
                padding: Sides::all(20.),
                direction: UiDirection::Horizontal,
                align: (Alignment::Center, Alignment::Center),
                size: (UiSize::Auto, UiSize::Auto),
                background: Some(vec4(0.1, 0.1, 0.1, 0.5)),
                corner_radius: Some(30.),
                elements: vec![
                  Box::new(Text {
                    text: "You tried".into(),
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
                ],
                ..Default::default()
              }),
            ],
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
