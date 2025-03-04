use derive_setters::Setters;
use glam::{vec2, Affine2};
use hui_painter::{paint::command::{PaintRectangle, PaintTransform}, texture::TextureHandle};
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{compute_size, Size, Size2d},
  measure::Response,
  rect::{Corners, FillColor},
};

#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct Image {
  /// Image handle to draw
  #[setters(skip)]
  pub image: TextureHandle,

  /// Size of the image.
  ///
  /// - If one of the dimensions is `Size::Auto`, the image will be scaled to fit the other dimension\
  ///   (aspect ratio is preserved)
  /// - If both dimensions are `Size::Auto`, the image will be drawn at its original size
  /// - All other values behave as expected
  #[setters(into)]
  pub size: Size2d,

  /// Color of the image
  ///
  /// Image will get multiplied/tinted by this color or gradient
  #[setters(into)]
  pub color: FillColor,

  /// Corner radius of the image
  #[setters(into)]
  pub corner_radius: Corners<f32>,
}

impl Image {
  pub fn new(handle: TextureHandle) -> Self {
    Self {
      image: handle,
      size: Size2d {
        width: Size::Auto,
        height: Size::Auto,
      },
      color: (1., 1., 1.).into(),
      corner_radius: Corners::all(0.),
    }
  }
}

impl UiElement for Image {
  fn name(&self) -> &'static str {
    "image"
  }

  fn size(&self) -> Option<Size2d> {
    Some(self.size)
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    let dim = self.image.size();
    let pre_size = compute_size(ctx.layout, self.size, dim.as_vec2());
    Response {
      size: compute_size(ctx.layout, self.size, vec2(
        match self.size.height {
          Size::Auto => dim.x as f32,
          _ => (pre_size.y / dim.y as f32) * dim.x as f32,
        },
        match self.size.height {
          Size::Auto => dim.x as f32,
          _ => (pre_size.y / dim.y as f32) * dim.x as f32,
        },
      )),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if !self.color.is_transparent() {
      ctx.paint_target.add(
        PaintTransform {
          transform: Affine2::from_translation(ctx.layout.position),
          child: PaintRectangle {
            size: ctx.measure.size,
            color: self.color,
            texture: Some(self.image),
            ..Default::default()
          },
        }
      );
    }
  }
}
