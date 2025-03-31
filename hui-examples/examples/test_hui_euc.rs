use minifb::{Window, WindowOptions};

pub fn main() {
  let mut window = Window::new(
    "hUI minfb (hui-euc)",
    800, 600,
    WindowOptions::default()
  ).unwrap();

  window.set_target_fps(60);

  // while window.is_open() {
  //   window
  //     .update_with_buffer(&buffer, WIDTH, HEIGHT)
  //     .unwrap();
  // }
}