// #[cfg(all(feature = "winit_30", feature = "winit_29"))]
// compile_error!("Only one of the winit_30 and winit_29 features can be enabled at a time");
// #[cfg(not(any(feature = "winit_30", feature = "winit_29")))]
// compile_error!("One of the winit_30 and winit_29 features must be enabled");
// #[cfg(feature = "winit_30")] extern crate winit_30 as winit;
// #[cfg(feature = "winit_29")] extern crate winit_29 as winit;

use glam::vec2;
use hui::{event::UiEvent, UiInstance};
use winit::event::{Event, WindowEvent, MouseButton, ElementState};

//TODO: check window id
pub fn handle_winit_event<T>(ui: &mut UiInstance, event: &Event<T>) {
  if let Event::WindowEvent { event, .. } = event {
    match event {
      WindowEvent::CursorMoved { position, .. } => {
        ui.push_event(UiEvent::MouseMove(vec2(position.x as f32, position.y as f32)));
      },
      WindowEvent::MouseInput { state, button, .. } => {
        ui.push_event(UiEvent::MouseButton {
          button: match button {
            MouseButton::Left => hui::input::MouseButton::Primary,
            MouseButton::Right => hui::input::MouseButton::Secondary,
            MouseButton::Middle => hui::input::MouseButton::Middle,
            MouseButton::Other(id) => hui::input::MouseButton::Other(*id as u8),
            _ => return,
          },
          state: match state {
            ElementState::Pressed => hui::input::ButtonState::Pressed,
            ElementState::Released => hui::input::ButtonState::Released,
          },
        })
      },
      //TODO: translate keyboard input
      _ => (),
    }
  }
}
