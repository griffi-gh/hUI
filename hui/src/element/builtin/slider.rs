//! a slider element that allows selecting a value in a range

use derive_setters::Setters;
use glam::{Vec2, vec2};

use crate::{
  draw::UiDrawCommand,
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{Size2d, compute_size},
  measure::Response,
  rect::FillColor,
  signal::{trigger::SignalTriggerArg, Signal},
};

/// Follow mode for the slider
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum SliderFollowMode {
  /// Slider will change based on the absolute mouse position in the slider
  ///
  /// This is the default mode and is recommended for most use cases
  #[default]
  Absolute,

  /// Slider will change based on the difference between the current and starting mouse position
  ///
  /// This is an experimental option and does not currently work well for sliders with large step sizes
  Relative,
}

/// A slider element that allows selecting a value in a range
#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct Slider {
  /// Value of the slider, should be in range 0..1
  ///
  /// Out of range values will be clamped
  pub value: f32,

  /// Size of the element
  #[setters(into)]
  pub size: Size2d,

  /// Color of the slider handle
  #[setters(into)]
  pub handle_color: FillColor,

  /// Color of the slider track
  #[setters(into)]
  pub track_color: FillColor,

  /// Follow mode
  pub follow_mode: SliderFollowMode,

  #[setters(skip)]
  pub on_change: Option<SignalTriggerArg<f32>>,
}

impl Default for Slider {
  fn default() -> Self {
    Self {
      value: 0.0,
      size: Size2d::default(),
      handle_color: (0.0, 0.0, 1.0).into(),
      track_color: (0.5, 0.5, 0.5).into(),
      follow_mode: SliderFollowMode::default(),
      on_change: None
    }
  }
}

impl Slider {
  pub const DEFAULT_HEIGHT: f32 = 21.0;

  pub fn new(value: f32) -> Self {
    Self {
      value,
      ..Default::default()
    }
  }

  pub fn on_change<S: Signal, T: Fn(f32) -> S + 'static>(self, f: T) -> Self {
    Self {
      on_change: Some(SignalTriggerArg::new(f)),
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
    //TODO: unhardcode this
    let bgrect_height_ratio = 0.33;

    ctx.draw.add(UiDrawCommand::Rectangle {
      position: ctx.layout.position + ctx.measure.size * vec2(0., 0.5 - bgrect_height_ratio / 2.),
      size: ctx.measure.size * vec2(1., bgrect_height_ratio),
      color: self.track_color.into(),
      texture: None,
      rounded_corners: None,
    });

    let value = self.value.clamp(0., 1.);
    let handle_size = vec2(15., ctx.measure.size.y);
    ctx.draw.add(UiDrawCommand::Rectangle {
      position: ctx.layout.position + ((ctx.measure.size.x - handle_size.x) * value) * Vec2::X,
      size: handle_size,
      color: self.handle_color.into(),
      texture: None,
      rounded_corners: None,
    });

    //handle events
    if let Some(res) = ctx.input.check_active(ctx.measure.rect(ctx.layout.position)) {
      let new_value = match self.follow_mode {
        SliderFollowMode::Absolute => ((res.position_in_rect.x - handle_size.x / 2.) / (ctx.measure.size.x - handle_size.x)).clamp(0., 1.),
        SliderFollowMode::Relative => {
          let delta = res.position_in_rect.x - res.last_position_in_rect.x;
          let delta_ratio = delta / (ctx.measure.size.x - handle_size.x);
          (self.value + delta_ratio).clamp(0., 1.)
        }
      };
      if let Some(signal) = &self.on_change {
        signal.fire(ctx.signal, new_value);
      }
      //TODO call signal with new value
    }
  }
}

//TODO
