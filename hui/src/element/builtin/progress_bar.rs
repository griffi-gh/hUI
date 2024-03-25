use derive_setters::Setters;
use glam::vec2;
use crate::{
  element::{MeasureContext, ProcessContext, UiElement},
  frame::{Frame, FrameRect},
  layout::{compute_size, Size, Size2d},
  measure::Response,
};

//TODO: Use Frames here instead of FillColor

#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct ProgressBar {
  /// Current progress, should be in the range 0.0..=1.0
  pub value: f32,

  /// Size of the progress bar element
  #[setters(into)]
  pub size: Size2d,

  /// Foreground (bar) color
  #[setters(skip)]
  pub foreground: Box<dyn Frame>,

  /// Background color
  #[setters(skip)]
  pub background: Box<dyn Frame>,
}

impl ProgressBar {
  pub const DEFAULT_HEIGHT: f32 = 20.0;

  pub fn with_background(mut self, frame: impl Frame + 'static) -> Self {
    self.background = Box::new(frame);
    self
  }

  pub fn with_foreground(mut self, frame: impl Frame + 'static) -> Self {
    self.foreground = Box::new(frame);
    self
  }
}

impl Default for ProgressBar {
  fn default() -> Self {
    Self {
      value: 0.,
      size: Size::Auto.into(),
      foreground: Box::new(FrameRect::color((0.0, 0.0, 1.0, 1.0))),
      background: Box::new(FrameRect::color((0.0, 0.0, 0.0, 1.0))),
    }
  }
}

impl UiElement for ProgressBar {
  fn name(&self) -> &'static str {
    "progress_bar"
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    Response {
      size: compute_size(ctx.layout, self.size, vec2(
        ctx.layout.max_size.x.max(300.), //XXX: remove .max(300)?
        Self::DEFAULT_HEIGHT,
      )),
      hints: Default::default(),
      user_data: None,
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    let value = self.value.clamp(0., 1.);

    //FIXME: these optimizations may not be valid
    if value < 1. || !self.foreground.covers_opaque() {
      self.background.draw(ctx.draw, ctx.layout.position, ctx.measure.size);
    }
    if value > 0. {
      self.foreground.draw(ctx.draw, ctx.layout.position, ctx.measure.size * vec2(value, 1.));
    }

    // let rounded_corners =
    //   (self.corner_radius.max_f32() > 0.).then_some({
    //     //HACK: fix clipping issues; //todo: get rid of this
    //     let mut radii = self.corner_radius;
    //     let width = ctx.measure.size.x * value;
    //     if width <= radii.max_f32() * 2. {
    //       radii.bottom_right = 0.;
    //       radii.top_right = 0.;
    //     }
    //     if width <= radii.max_f32() {
    //       radii.bottom_left = 0.;
    //       radii.top_left = 0.;
    //     }
    //     RoundedCorners::from_radius(radii)
    //   });
    // if value < 1. {
    //   ctx.draw.add(UiDrawCommand::Rectangle {
    //     position: ctx.layout.position,
    //     size: ctx.measure.size,
    //     color: self.background.corners(),
    //     texture: None,
    //     texture_uv: None,
    //     rounded_corners
    //   });
    // }
    // if value > 0. {
    //   ctx.draw.add(UiDrawCommand::Rectangle {
    //     position: ctx.layout.position,
    //     size: ctx.measure.size * vec2(value, 1.0),
    //     color: self.foreground.corners(),
    //     texture: None,
    //     texture_uv: None,
    //     rounded_corners,
    //   });
    // }
  }
}
