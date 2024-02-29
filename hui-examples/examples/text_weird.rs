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
    container::Container, fill_rect::FillRect, spacer::Spacer, text::Text, ElementList
  }, layout::Size, UiInstance
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
  window.set_title("Text rendering test");

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

        hui.add(Container {
          size: (Size::Fraction(1.), Size::Fraction(1.)).into(),
          background: vec4(0.1, 0.1, 0.1, 1.).into(),
          children: elements(|elem| {
            elem.push(Box::new(Text {
              text: "THIS LINE SHOULD BE SHARP!".into(),
              ..Default::default()
            }));
            elem.push(Box::new(Text {
              text: "THIS LINE SHOULD BE SHARP!".into(),
              text_size: 32,
              ..Default::default()
            }));
            elem.push(Box::new(Text {
              text: "All lines except 3 and 6 below will be blurry:".into(),
              ..Default::default()
            }));
            for size in [9, 12, 16, 18, 24, 32] {
              elem.push(Box::new(Text {
                text: "Testing default font, Proggy Tiny".into(),
                text_size: size,
                ..Default::default()
              }));
            }
            elem.push(Box::new(FillRect {
              size: (Size::Fraction(1.), Size::Static(10.)).into(),
              background: vec4(0., 0., 1., 1.).into(),
              ..Default::default()
            }));
            elem.push(Box::new(FillRect {
              size: (Size::Fraction(1.), Size::Static(10.)).into(),
              background: vec4(1., 1., 0., 1.).into(),
              ..Default::default()
            }));
            elem.push(Box::new(Text {
              text: "Hello, world!\nżółty liść. życie nie ma sensu i wszyscy zginemy;\nтест кирилиці їїїїїїїїїїї\njapanese text: テスト".into(),
              font: font_handle,
              text_size: 32,
              ..Default::default()
            }));
            if instant.elapsed().as_secs() & 1 != 0 {
              elem.push(Box::new(FillRect {
                size: (Size::Fraction(1.), Size::Static(10.)).into(),
                background: vec4(1., 0., 0., 1.).into(),
                ..Default::default()
              }));
              elem.push(Box::new(FillRect {
                size: (Size::Fraction(1.), Size::Static(10.)).into(),
                background: vec4(0., 0., 0., 1.).into(),
                ..Default::default()
              }));
              elem.push(Box::new(Spacer(100.)));
              elem.push(Box::new(Text {
                text: "FLAG SHOULD NOT OVERLAP WITH TEXT".into(),
                text_size: 64,
                color: vec4(1., 0., 1., 1.),
                ..Default::default()
              }));
            }
          }),
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
