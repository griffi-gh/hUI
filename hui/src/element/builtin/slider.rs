//! work in progress

use derive_setters::Setters;

use crate::{
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{compute_size, Size2d},
  measure::Response,
  rectangle::Corners,
  signal::UiSignal,
};

/// work in progress
#[derive(Default, Debug, Clone, Copy, Setters)]
#[setters(prefix = "with_")]
pub struct Slider {
  pub value: f32,
  pub size: Size2d,
}

impl Slider {
  pub const DEFAULT_HEIGHT: f32 = 20.0;

  pub fn new(value: f32) -> Self {
    Self {
      value,
      ..Default::default()
    }
  }
}

impl UiElement for Slider {
  fn name(&self) -> &'static str {
    "Slider"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: compute_size(ctx.layout, self.size, (ctx.layout.max_size.x, Self::DEFAULT_HEIGHT).into()),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    ctx.draw.add(UiDrawCommand::Rectangle {
      position: ctx.layout.position,
      size: ctx.measure.size,
      color: Corners::all((1., 0., 0., 1.).into()),
      texture: None,
      rounded_corners: None,
    });

    let value = self.value.clamp(0., 1.);
    ctx.draw.add(UiDrawCommand::Rectangle {
      position: ctx.layout.position,
      size: (ctx.measure.size.x * value, ctx.measure.size.y).into(),
      color: Corners::all((0., 1., 0., 1.).into()),
      texture: None,
      rounded_corners: None,
    });

    //handle click etc
    if let Some(res) = ctx.input.check_click(ctx.measure.rect(ctx.layout.position)) {
      let new_value = res.position_in_rect.x / ctx.measure.size.x;
      //TODO call signal with new value
    }
  }
}

//TODO
