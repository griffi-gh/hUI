//! wrapper that allows applying various transformations to an element, such as translation, rotation, or scaling

use glam::{Affine2, Vec2};
use hui_painter::paint::command::{PaintList, PaintTransform};
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  measure::Response,
};

pub struct Transformer {
  pub transform: Affine2,
  pub element: Box<dyn UiElement>,
}

/// Wrapper that allows applying various transformations to an element, such as translation, rotation, or scaling\
/// Use sparingly, as this is an experimental feature and may not work as expected\
impl Transformer {
  pub fn new(element: Box<dyn UiElement>) -> Self {
    Self {
      transform: Affine2::IDENTITY,
      element,
    }
  }

  pub fn translate(mut self, v: impl Into<Vec2>) -> Self {
    self.transform *= Affine2::from_translation(v.into());
    self
  }

  pub fn scale(mut self, v: impl Into<Vec2>) -> Self {
    self.transform *= Affine2::from_scale(v.into());
    self
  }

  pub fn rotate(mut self, radians: f32) -> Self {
    self.transform *= Affine2::from_angle(radians);
    self
  }
}

impl UiElement for Transformer {
  fn name(&self) -> &'static str {
    "transformer"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    self.element.measure(ctx)
  }

  fn process(&self, ctx: ProcessContext) {
    if self.transform == Affine2::IDENTITY {
      self.element.process(ctx);
      return;
    }

    let mut sub_list = PaintList::new_empty();
    self.element.process(ProcessContext {
      painter: ctx.painter,
      measure: ctx.measure,
      state: ctx.state,
      layout: ctx.layout,
      paint_target: &mut sub_list,
      current_font: ctx.current_font,
      input: ctx.input,
      signal: ctx.signal,
    });

    ctx.paint_target.add(PaintTransform {
      transform: self.transform,
      child: sub_list,
    });
  }
}

/// Extension trait for [`UiElement`] that adds the [`transform`] method
pub trait ElementTransformExt {
  /// Wrap the element in a [`Transformer`]
  ///
  /// This allows you to apply various transformations to the element, such as translation, rotation, or scaling\
  /// Use sparingly, as this is an experimental feature and may not work as expected\
  /// Transform is applied around the center of the element's bounding box.
  fn transform(self) -> Transformer;
}

impl<T: UiElement + 'static> ElementTransformExt for T {
  fn transform(self) -> Transformer {
    Transformer::new(Box::new(self))
  }
}
