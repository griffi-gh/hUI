//! work in progress

use derive_setters::Setters;
use glam::{vec2, Vec2};

use crate::{
  draw::{RoundedCorners, UiDrawCommand},
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{compute_size, Size2d},
  measure::Response,
  rectangle::Corners,
  signal::{SignalStore, UiSignal},
};

/// work in progress
#[derive(Default, Setters)]
#[setters(prefix = "with_")]
pub struct Slider {
  pub value: f32,
  pub size: Size2d,

  #[setters(skip)]
  fire_on_shit: Option<Box<dyn Fn(&mut SignalStore, f32)>>,
}

impl Slider {
  pub const DEFAULT_HEIGHT: f32 = 20.0;

  pub fn new(value: f32) -> Self {
    Self {
      value,
      ..Default::default()
    }
  }

  pub fn on_change<S: UiSignal + 'static, T: Fn(f32) -> S + 'static>(self, f: T) -> Self {
    Self {
      fire_on_shit: Some(Box::new(move |s: &mut SignalStore, x| {
        s.add::<S>(f(x));
      })),
      ..self
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
    let bgrect_height_ratio = 0.25;
    ctx.draw.add(UiDrawCommand::Rectangle {
      position: ctx.layout.position + ctx.measure.size * vec2(0., 0.5 - bgrect_height_ratio / 2.),
      size: ctx.measure.size * vec2(1., bgrect_height_ratio),
      color: Corners::all((1., 1., 1., 0.7).into()),
      texture: None,
      rounded_corners: None,
      //Some(RoundedCorners::from_radius(Corners::all(bgrect_height_ratio * ctx.measure.size.y * 0.4))),
    });

    let value = self.value.clamp(0., 1.);
    let handle_size = vec2(15., ctx.measure.size.y);
    ctx.draw.add(UiDrawCommand::Rectangle {
      position: ctx.layout.position + (ctx.measure.size.x * value - handle_size.x / 2.) * Vec2::X,
      size: handle_size,
      color: Corners::all((1., 1., 1., 1.).into()),
      texture: None,
      rounded_corners: None,
      //Some(RoundedCorners::from_radius(Corners::all(handle_size.x / 3.))),
    });

    //handle click etc
    if let Some(res) = ctx.input.check_active(ctx.measure.rect(ctx.layout.position)) {
      let new_value = (res.position_in_rect.x / ctx.measure.size.x).clamp(0., 1.);
      if let Some(fire) = &self.fire_on_shit {
        fire(ctx.signal, new_value);
      }
      //TODO call signal with new value
    }
  }
}

//TODO
