use glam::vec2;
use hui::{event::UiEvent, UiInstance};
use winit::event::{Event, WindowEvent};

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
            winit::event::MouseButton::Left => hui::input::MouseButton::Primary,
            winit::event::MouseButton::Right => hui::input::MouseButton::Secondary,
            winit::event::MouseButton::Middle => hui::input::MouseButton::Middle,
            winit::event::MouseButton::Other(id) => hui::input::MouseButton::Other(*id as u8),
            _ => return,
          },
          state: match state {
            winit::event::ElementState::Pressed => hui::input::ButtonState::Pressed,
            winit::event::ElementState::Released => hui::input::ButtonState::Released,
          },
        })
      },
      //TODO: translate keyboard input
      _ => (),
    }
  }
}
