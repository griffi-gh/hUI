use glam::{vec2, Vec4, vec4};
use crate::{
  draw::{RoundedCorners, UiDrawCommand}, element::{MeasureContext, ProcessContext, UiElement}, layout::UiSize, measure::Response, rectangle::Corners
};

#[derive(Debug, Clone, Copy)]
pub struct ProgressBar {
  pub size: (UiSize, UiSize),
  pub value: f32,
  pub color_foreground: Vec4,
  pub color_background: Vec4,
  pub corner_radius: Corners<f32>,
}

impl ProgressBar {
  pub const DEFAULT_HEIGHT: f32 = 20.0;
}

impl Default for ProgressBar {
  fn default() -> Self {
    Self {
      size: (UiSize::Auto, UiSize::Auto),
      value: 0.,
      color_foreground: vec4(0.0, 0.0, 1.0, 1.0),
      color_background: vec4(0.0, 0.0, 0.0, 1.0),
      corner_radius: Corners::all(0.),
    }
  }
}

impl UiElement for ProgressBar {
  fn name(&self) -> &'static str { "Progress bar" }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: vec2(
        match self.size.0 {
          UiSize::Auto => ctx.layout.max_size.x.max(300.),
          UiSize::Fraction(p) => ctx.layout.max_size.x * p,
          UiSize::Static(p) => p,
        },
        match self.size.1 {
          UiSize::Auto => Self::DEFAULT_HEIGHT,
          UiSize::Fraction(p) => ctx.layout.max_size.y * p,
          UiSize::Static(p) => p,
        }
      ),
      hints: Default::default(),
      user_data: None,
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
        color: Corners::all(self.color_background),
        rounded_corners
      });
    }
    if value > 0. {
      ctx.draw.add(UiDrawCommand::Rectangle {
        position: ctx.layout.position,
        size: ctx.measure.size * vec2(value, 1.0),
        color: Corners::all(self.color_foreground),
        rounded_corners,
      });
    }
  }
}
