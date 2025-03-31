#![cfg_attr(not(feature = "std"), no_std)]

// TODO

use euc::Pipeline;

struct UiPipeline;

impl Pipeline for UiPipeline {
  type Vertex = [f32; 2];
  type VsOut = ();
  type Pixel = [u8; 4];

  // Vertex shader
  fn vert(&self, pos: &Self::Vertex) -> ([f32; 4], Self::VsOut) {
    ([pos[0], pos[1], 0.0, 1.0], ())
  }

  // Fragment shader
  fn frag(&self, _: &Self::VsOut) -> Self::Pixel {
    [0, 0, 0, 255]
  }
}

// fn main() {
//   let mut color = Buffer2d::new([640, 480], [0; 4]);
//   let mut depth = Buffer2d::new([640, 480], 1.0);

//   Example.draw::<Triangles<_>, _>(
//     &[[-1.0, -1.0], [1.0, -1.0], [0.0, 1.0]],
//     &mut color,
//     &mut depth,
//   );
// }
