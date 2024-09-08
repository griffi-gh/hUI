use glam::{UVec2, Vec2};
use glium::{Surface, backend::glutin::SimpleWindowBuilder};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::UiInstance;
use hui_glium::GliumUiRenderer;

/// Generates a `main` function that initializes glium renderer, `UiInstance`, and runs the event loop.
macro_rules! ui_main {
  ($name:literal, init: $closure0:expr, run: $closure1:expr) => {
    fn main() {
      $crate::boilerplate::ui($closure0, $closure1, $name);
    }
  };
  (init: $closure0:expr, run: $closure1:expr) => {
    fn main() {
      $crate::boilerplate::ui($closure0, $closure1, "hUI example");
    }
  };
  ($closure: expr) => {
    fn main() {
      $crate::boilerplate::ui(|_|(), $closure, "hUI example");
    }
  };
}

/// Initializes glium renderer, `UiInstance`, and runs the event loop.
pub fn ui<T>(
  mut init: impl FnMut(&mut UiInstance) -> T,
  mut draw: impl FnMut(&mut UiInstance, Vec2, &mut T),
  name: &'static str
) {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (window, display) = SimpleWindowBuilder::new()
    .with_title(name)
    .build(&event_loop);

  let mut hui = UiInstance::new();
  let mut backend = GliumUiRenderer::new(&display);

  let mut result = init(&mut hui);

  event_loop.run(|event, window_target| {
    window.request_redraw();
    window_target.set_control_flow(ControlFlow::Poll);
    hui_winit::handle_winit_event(&mut hui, &event);
    match event {
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          window_target.exit();
        },
        WindowEvent::Resized(size) => {
          display.resize((size.width, size.height));
        },
        WindowEvent::RedrawRequested => {
          let mut frame = display.draw();
          frame.clear_color_srgb(0.5, 0.5, 0.5, 1.);

          hui.begin();

          let size = UVec2::from(display.get_framebuffer_dimensions()).as_vec2();
          draw(&mut hui, size, &mut result);

          hui.end();

          backend.update(&hui);
          backend.draw(&mut frame, size);

          frame.finish().unwrap();
        },
        _ => (),
      },
      Event::Suspended => {
        #[cfg(target_os = "android")]
        window_target.exit();
      },
      _ => (),
    }
  }).unwrap();
}
