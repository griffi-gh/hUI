use derive_setters::Setters;
use glam::vec2;
use crate::{
  background::BackgroundColor,
  draw::{ImageHandle, RoundedCorners, UiDrawCommand},
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{Size, Size2d},
  measure::Response,
  rectangle::Corners,
};

#[derive(Setters)]
pub struct Image {
  /// Image handle to draw
  #[setters(skip)]
  pub image: ImageHandle,

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
  pub color: BackgroundColor,

  /// Corner radius of the image
  #[setters(into)]
  pub corner_radius: Corners<f32>,
}

impl Image {
  pub fn new(handle: ImageHandle) -> Self {
    Self {
      image: handle,
      size: Size2d {
        width: Size::Auto,
        height: Size::Auto,
      },
      color: BackgroundColor::from((1., 1., 1., 1.)),
      corner_radius: Corners::all(0.),
    }
  }
}

impl UiElement for Image {
  fn name(&self) -> &'static str {
    "Image"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    let dim = ctx.images.get_size(self.image).expect("invalid image handle");
    Response {
      size: vec2(
        match self.size.width {
          Size::Auto => {
            match self.size.height {
              Size::Auto => dim.x as f32,
              Size::Fraction(f) => ((f * ctx.layout.max_size.y) / dim.y as f32) * dim.x as f32,
              Size::Static(pixels) => (pixels / dim.y as f32) * dim.x as f32,
            }
          },
          Size::Fraction(percentage) => ctx.layout.max_size.x * percentage,
          Size::Static(pixels) => pixels,
        },
        match self.size.height {
          Size::Auto => {
            match self.size.width {
              Size::Auto => dim.y as f32,
              Size::Fraction(f) => ((f * ctx.layout.max_size.x) / dim.x as f32) * dim.y as f32,
              Size::Static(pixels) => (pixels / dim.x as f32) * dim.y as f32,
            }
          },
          Size::Fraction(percentage) => ctx.layout.max_size.y * percentage,
          Size::Static(pixels) => pixels,
        },
      ),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    if !self.color.is_transparent() {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color: self.color.corners(),
        texture: Some(self.image),
        rounded_corners: (self.corner_radius.max_f32() > 0.).then_some({
          RoundedCorners::from_radius(self.corner_radius)
        }),
      });
    }
  }
}
