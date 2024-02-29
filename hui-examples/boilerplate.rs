use glam::{UVec2, Vec2};
use glium::{backend::glutin::SimpleWindowBuilder, Surface};
use winit::{
  event::{Event, WindowEvent},
  event_loop::{EventLoopBuilder, ControlFlow}
};
use hui::UiInstance;
use hui_glium::GliumUiRenderer;

/// Generates a `main` function that initializes glium renderer, `UiInstance`, and runs the event loop.
macro_rules! ui_main {
  ($closure: expr) => {
    fn main() {
      $crate::boilerplate::ui($closure);
    }
  };
}

/// Initializes glium renderer, `UiInstance`, and runs the event loop.
pub fn ui(mut x: impl FnMut(&mut UiInstance, Vec2)) {
  kubi_logging::init();

  let event_loop = EventLoopBuilder::new().build().unwrap();
  let (_window, display) = SimpleWindowBuilder::new().build(&event_loop);

  let mut hui = UiInstance::new();
  let mut backend = GliumUiRenderer::new(&display);

  event_loop.run(|event, window_target| {
    window_target.set_control_flow(ControlFlow::Poll);
    hui_winit::handle_winit_event(&mut hui, &event);
    match event {
      Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
        window_target.exit();
      },
      Event::AboutToWait => {
        let mut frame = display.draw();
        frame.clear_color_srgb(0.5, 0.5, 0.5, 0.);

        hui.begin();

        let size = UVec2::from(display.get_framebuffer_dimensions()).as_vec2();
        x(&mut hui, size);

        hui.end();

        backend.update(&hui);
        backend.draw(&mut frame, size);

        frame.finish().unwrap();
      }
      _ => (),
    }
  }).unwrap();
}
