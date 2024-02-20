use glam::vec2;
use hui::{event::UiEvent, UiInstance};
use winit::event::{Event, WindowEvent};

//TODO: window id
pub fn handle_winit_event<T>(ui: &mut UiInstance, event: &Event<T>) {
  if let Event::WindowEvent { event, .. } = event {
    match event {
      WindowEvent::CursorMoved { position, .. } => {
        ui.push_event(UiEvent::MouseMove(vec2(position.x as f32, position.y as f32)));
      },
      WindowEvent::MouseInput { state, button, .. } => {
        ui.push_event(UiEvent::MouseButton {
          button: match button {
            winit::event::MouseButton::Left => hui::event::MouseButton::Primary,
            winit::event::MouseButton::Right => hui::event::MouseButton::Secondary,
            winit::event::MouseButton::Middle => hui::event::MouseButton::Middle,
            winit::event::MouseButton::Other(id) => hui::event::MouseButton::Other(*id as u8),
            _ => return,
          },
          state: match state {
            winit::event::ElementState::Pressed => hui::event::ButtonState::Pressed,
            winit::event::ElementState::Released => hui::event::ButtonState::Released,
          },
        })
      },
      //TODO keyboard
      _ => (),
    }
  }
}
