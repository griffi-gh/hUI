use derive_setters::Setters;
use glam::{vec2, vec4};
use crate::{
  background::BackgroundColor,
  draw::{RoundedCorners, UiDrawCommand},
  element::{MeasureContext, ProcessContext, UiElement},
  layout::{compute_size, Size, Size2d},
  measure::Response,
  rectangle::Corners
};

#[derive(Debug, Clone, Copy, Setters)]
#[setters(prefix = "with_")]
pub struct ProgressBar {
  /// Current progress, should be in the range 0.0..=1.0
  pub value: f32,

  /// Size of the progress bar element
  #[setters(into)]
  pub size: Size2d,

  /// Foreground (bar) color
  #[setters(into)]
  pub foreground: BackgroundColor,

  /// Background color
  #[setters(into)]
  pub background: BackgroundColor,

  /// Corner radius of the progress bar
  #[setters(into)]
  pub corner_radius: Corners<f32>,
}

impl ProgressBar {
  pub const DEFAULT_HEIGHT: f32 = 20.0;
}

impl Default for ProgressBar {
  fn default() -> Self {
    Self {
      value: 0.,
      size: Size::Auto.into(),
      foreground: vec4(0.0, 0.0, 1.0, 1.0).into(),
      background: vec4(0.0, 0.0, 0.0, 1.0).into(),
      corner_radius: Corners::all(0.),
    }
  }
}

impl UiElement for ProgressBar {
  fn name(&self) -> &'static str {
    "ProgressBar"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: compute_size(ctx.layout, self.size, vec2(
        ctx.layout.max_size.x.max(300.),
        Self::DEFAULT_HEIGHT,
      )),
      hints: Default::default(),
      user_data: None,
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    let value = self.value.clamp(0., 1.);
    let rounded_corners =
      (self.corner_radius.max_f32() > 0.).then_some({
        //HACK: fix clipping issues; //todo: get rid of this
        let mut radii = self.corner_radius;
        let width = ctx.measure.size.x * value;
        if width <= radii.max_f32() * 2. {
          radii.bottom_right = 0.;
          radii.top_right = 0.;
        }
        if width <= radii.max_f32() {
          radii.bottom_left = 0.;
          radii.top_left = 0.;
        }
        RoundedCorners::from_radius(radii)
      });
    if value < 1. {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size,
        color: self.background.corners(),
        texture: None,
        rounded_corners
      });
    }
    if value > 0. {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size * vec2(value, 1.0),
        color: self.foreground.corners(),
        texture: None,
        rounded_corners,
      });
    }
  }
}
