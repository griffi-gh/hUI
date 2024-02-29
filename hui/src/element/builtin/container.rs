use derive_setters::Setters;
use glam::{Vec2, vec2};
use crate::{
  background::RectBackground,
  draw::{RoundedCorners, UiDrawCommand},
  element::{ElementList, MeasureContext, ProcessContext, UiElement},
  layout::{Alignment, Alignment2d, LayoutInfo, UiDirection, Size, Size2d},
  measure::{Hints, Response},
  rectangle::{Corners, Sides}
};

// pub struct Border {
//   pub color: Vec4,
//   pub width: f32,
// }

///XXX: add Order/Direction::Forward/Reverse or sth?
//TODO: clip children flag
//TODO: borders
//TODO: min/max size

#[derive(Setters)]
#[setters(prefix = "with_")]
pub struct Container {
  #[setters(into)]
  pub size: Size2d,
  pub direction: UiDirection,
  pub gap: f32,
  #[setters(into)]
  pub padding: Sides<f32>,
  #[setters(into)]
  pub align: Alignment2d,
  #[setters(into)]
  pub background: RectBackground,
  #[setters(into)]
  pub corner_radius: Corners<f32>,
  #[setters(skip)]
  pub children: ElementList,
}

impl Container {
  pub fn with_children(mut self, ui: impl FnOnce(&mut ElementList)) -> Self {
    self.children.0.extend(ElementList::from_callback(ui).0);
    self
  }
}

impl Default for Container {
  fn default() -> Self {
    Self {
      size: (Size::Auto, Size::Auto).into(),
      direction: UiDirection::Vertical,
      gap: 0.,
      padding: Sides::all(0.),
      align: Alignment2d::default(),
      background: Default::default(),
      children: ElementList(Vec::new()),
      corner_radius: Corners::all(0.),
    }
  }
}

impl Container {
  pub fn measure_max_inner_size(&self, layout: &LayoutInfo) -> Vec2 {
    let outer_size_x = match self.size.width {
      Size::Auto => layout.max_size.x,
      Size::Fraction(p) => layout.max_size.x * p,
      Size::Static(p) => p,
    };
    let outer_size_y = match self.size.height {
      Size::Auto => layout.max_size.y,
      Size::Fraction(p) => layout.max_size.y * p,
      Size::Static(p) => p,
    };
    vec2(
      outer_size_x - (self.padding.left + self.padding.right),
      outer_size_y - (self.padding.top + self.padding.bottom),
    )
  }
}

impl UiElement for Container {
  fn measure(&self, ctx: MeasureContext) -> Response {
    let mut size = Vec2::ZERO;
    let mut leftover_gap = Vec2::ZERO;
    for element in &self.children.0 {
      let measure = element.measure(MeasureContext{
        state: ctx.state,
        layout: &LayoutInfo {
          position: ctx.layout.position + size,
          max_size: self.measure_max_inner_size(ctx.layout), // - size TODO
          direction: self.direction,
        },
        text_measure: ctx.text_measure,
      });
      match self.direction {
        UiDirection::Horizontal => {
          size.x += measure.size.x + self.gap;
          size.y = size.y.max(measure.size.y);
          leftover_gap.x = self.gap;
        },
        UiDirection::Vertical => {
          size.x = size.x.max(measure.size.x);
          size.y += measure.size.y + self.gap;
          leftover_gap.y = self.gap;
        }
      }
    }
    size -= leftover_gap;

    let inner_content_size = Some(size);

    size += vec2(
      self.padding.left + self.padding.right,
      self.padding.top + self.padding.bottom,
    );

    match self.size.width {
      Size::Auto => (),
      Size::Fraction(percentage) => size.x = ctx.layout.max_size.x * percentage,
      Size::Static(pixels) => size.x = pixels,
    }
    match self.size.height {
      Size::Auto => (),
      Size::Fraction(percentage) => size.y = ctx.layout.max_size.y * percentage,
      Size::Static(pixels) => size.y = pixels,
    }

    Response {
      size,
      hints: Hints {
        inner_content_size,
        ..Default::default()
      },
      user_data: None
    }
  }

  fn process(&self, ctx: ProcessContext) {
    let mut position = ctx.layout.position;

    //background
    if !self.background.is_transparent() {
      let corner_colors = self.background.corners().unwrap();
      ctx.draw.add(UiDrawCommand::Rectangle {
        position,
        size: ctx.measure.size,
        color: corner_colors,
        texture: None,
        rounded_corners: (self.corner_radius.max_f32() > 0.).then_some({
          RoundedCorners::from_radius(self.corner_radius)
        }),
      });
    }

    //padding
    position += vec2(self.padding.left, self.padding.top);

    //convert alignment to pri/sec axis based
    //.0 = primary, .1 = secondary
    let pri_sec_align = match self.direction {
      UiDirection::Horizontal => (self.align.horizontal, self.align.vertical),
      UiDirection::Vertical => (self.align.horizontal, self.align.vertical),
    };

    //alignment
    match (pri_sec_align.0, self.direction) {
      (Alignment::Begin, _) => (),
      (Alignment::Center, UiDirection::Horizontal) => {
        position.x += (ctx.measure.size.x - ctx.measure.hints.inner_content_size.unwrap().x) / 2. - self.padding.left;
      },
      (Alignment::Center, UiDirection::Vertical) => {
        position.y += (ctx.measure.size.y - ctx.measure.hints.inner_content_size.unwrap().y) / 2. - self.padding.top;
      },
      (Alignment::End, UiDirection::Horizontal) => {
        position.x += ctx.measure.size.x - ctx.measure.hints.inner_content_size.unwrap().x - self.padding.right - self.padding.left;
      },
      (Alignment::End, UiDirection::Vertical) => {
        position.y += ctx.measure.size.y - ctx.measure.hints.inner_content_size.unwrap().y - self.padding.bottom - self.padding.top;
      }
    }

    for element in &self.children.0 {
      //(passing max size from layout rather than actual bounds for the sake of consistency with measure() above)

      let mut el_layout = LayoutInfo {
        position,
        max_size: self.measure_max_inner_size(ctx.layout),
        direction: self.direction,
      };

      //measure
      let el_measure = element.measure(MeasureContext {
        state: ctx.state,
        layout: &el_layout,
        text_measure: ctx.text_measure,
      });

      //align (on sec. axis)
      match (pri_sec_align.1, self.direction) {
        (Alignment::Begin, _) => (),
        (Alignment::Center, UiDirection::Horizontal) => {
          el_layout.position.y += (ctx.measure.size.y - self.padding.bottom - self.padding.top - el_measure.size.y) / 2.;
        },
        (Alignment::Center, UiDirection::Vertical) => {
          el_layout.position.x += (ctx.measure.size.x - self.padding.left - self.padding.right - el_measure.size.x) / 2.;
        },
        (Alignment::End, UiDirection::Horizontal) => {
          el_layout.position.y += ctx.measure.size.y - el_measure.size.y - self.padding.bottom;
        },
        (Alignment::End, UiDirection::Vertical) => {
          el_layout.position.x += ctx.measure.size.x - el_measure.size.x - self.padding.right;
        }
      }

      //process
      element.process(ProcessContext {
        measure: &el_measure,
        state: ctx.state,
        layout: &el_layout,
        draw: ctx.draw,
        text_measure: ctx.text_measure,
      });

      //layout
      match self.direction {
        UiDirection::Horizontal => {
          position.x += el_measure.size.x + self.gap;
        },
        UiDirection::Vertical => {
          position.y += el_measure.size.y + self.gap;
        }
      }
    }
  }
}
