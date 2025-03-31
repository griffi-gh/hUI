//! a container element that can hold and layout multiple children elements

use alloc::{boxed::Box, vec::Vec};
use derive_setters::Setters;
use glam::{Vec2, vec2};
use crate::{
  element::{ElementList, MeasureContext, ProcessContext, UiElement},
  frame::{Frame, RectFrame},
  layout::{compute_size, Alignment, Alignment2d, Direction, LayoutInfo, Size, Size2d, WrapBehavior},
  measure::{Hints, Response},
  rect::Sides,
};

//XXX: add Order/Direction::Forward/Reverse or sth?
//TODO: clip children flag
//TODO: borders
//TODO: min/max size

#[derive(Clone, Copy)]
struct CudLine {
  start_idx: usize,
  content_size: Vec2,
  remaining_space: f32,
}

struct ContainerUserData {
  lines: Vec<CudLine>,
}

/// A container element that can hold and layout multiple children elements
#[derive(Setters)]
#[setters(no_std, prefix = "with_")]
pub struct Container {
  /// Size of the container
  #[setters(into)]
  pub size: Size2d,

  /// Layout direction (horizontal/vertical)
  pub direction: Direction,

  //XXX: should we have separate gap value for primary and secondary (when wrapped, between lines of elements) axis?

  /// Gap between children elements
  pub gap: f32,

  /// Padding inside the container (distance from the edges to the children elements)
  #[setters(into)]
  pub padding: Sides<f32>,

  /// Margin outside the container (distance from the edges to the parent element)
  #[setters(into)]
  pub margin: Sides<f32>,

  /// Alignment of the children elements on X and Y axis
  #[setters(into)]
  pub align: Alignment2d,

  #[setters(skip)]
  pub background_frame: Box<dyn Frame>,

  /// Controls if wrapping is enabled
  #[setters(into)]
  pub wrap: WrapBehavior,

  /// List of children elements
  #[setters(skip)]
  pub children: ElementList,
}

impl Container {
  pub fn with_children(mut self, ui: impl FnOnce(&mut ElementList)) -> Self {
    self.children.0.extend(ElementList::from_callback(ui).0);
    self
  }

  pub fn with_background(mut self, frame: impl Frame + 'static) -> Self {
    self.background_frame = Box::new(frame);
    self
  }
}

impl Default for Container {
  fn default() -> Self {
    Self {
      size: (Size::Auto, Size::Auto).into(),
      direction: Direction::Vertical,
      gap: 0.,
      padding: Sides::all(0.),
      margin: Sides::all(0.),
      align: Alignment2d::default(),
      background_frame: Box::<RectFrame>::default(),
      wrap: WrapBehavior::Allow,
      children: ElementList(Vec::new()),
    }
  }
}

impl Container {
  pub fn measure_max_inner_size(&self, layout: &LayoutInfo) -> Vec2 {
    // let outer_size_x = match self.size.width {
    //   Size::Auto => layout.max_size.x,
    //   Size::Relative(p) => layout.max_size.x * p,
    //   Size::Absolute(p) => p,
    //   Size::Remaining(p) => match layout.direction {
    //     Direction::Horizontal => layout.remaining_space.unwrap_or(layout.max_size.x) * p,
    //     Direction::Vertical => layout.max_size.x,
    //   }
    // };
    // let outer_size_y = match self.size.height {
    //   Size::Auto => layout.max_size.y,
    //   Size::Relative(p) => layout.max_size.y * p,
    //   Size::Absolute(p) => p,
    // };
    let outer_size = compute_size(layout, self.size, layout.max_size);
    vec2(
      outer_size.x - self.padding.sum_horizontal() - self.margin.sum_horizontal(),
      outer_size.y - self.padding.sum_vertical() - self.margin.sum_vertical(),
    )
  }
}

impl UiElement for Container {
  fn name(&self) -> &'static str {
    "container"
  }

  fn size(&self) -> Option<Size2d> {
    Some(self.size)
  }

  fn measure(&self, ctx: MeasureContext) -> Response {
    // XXX: If both axes are NOT set to auto, we should be able quickly return the size
    // ... but we can't, because we need to measure the children to get the inner_content_size and user_data values
    // this is a potential optimization opportunity, maybe we could postpone this to the process call
    // as it's guaranteed to be called only ONCE, while measure is assumed to be cheap and called multiple times
    // ... we could also implement some sort of "global" caching for the measure call (to prevent traversal of the same tree multiple times),
    // but that's a bit more complex and probably impossible with the current design of the measure/process calls

    // In case wrapping is enabled, elements cannot exceed this size on the primary axis
    let max_line_pri = match self.direction {
      Direction::Horizontal => match self.size.width {
        Size::Auto => ctx.layout.max_size.x,
        Size::Relative(p) => ctx.layout.max_size.x * p,
        Size::Absolute(p) => p,
        Size::Remaining(p) => ctx.layout.remaining_space.unwrap_or(ctx.layout.max_size.x) * p,
      },
      Direction::Vertical => match self.size.height {
        Size::Auto => ctx.layout.max_size.y,
        Size::Relative(p) => ctx.layout.max_size.y * p,
        Size::Absolute(p) => p,
        Size::Remaining(p) => ctx.layout.remaining_space.unwrap_or(ctx.layout.max_size.y) * p,
      }
    };

    let padding_with_margin = self.padding + self.margin;

    //size of AABB containing all lines
    let mut total_size = Vec2::ZERO;

    //Size of the current row/column (if wrapping)
    let mut line_size = Vec2::ZERO;

    //Size of previous sec. axes combined
    //(basically, in case of the horizontal layout, this is the height of the tallest element in the line)
    //This is a vec2, but only one axis is used, depending on the layout direction
    let mut line_sec_offset: Vec2 = Vec2::ZERO;

    //Amount of elements in the current line
    let mut line_element_count = 0;

    //Leftover gap from the previous element on the primary axis
    let mut leftover_gap = Vec2::ZERO;

    //line metadata for the user_data
    let mut lines = vec![
      CudLine {
        start_idx: 0,
        content_size: Vec2::ZERO,
        remaining_space: 0.,
      }
    ];

    //set to true if in the current line there is an element with Remaining size (line will have to be wrapped)
    // let mut has_remaining = false;

    for (idx, element) in self.children.0.iter().enumerate() {
      if let Some(esize) = element.size() {
        let pri_size = match self.direction {
          Direction::Horizontal => esize.width,
          Direction::Vertical => esize.height,
        };
        if matches!(pri_size, Size::Remaining(_)) {
          //XXX: kinda a hack?
          continue;
        }
      }

      let measure = element.measure(MeasureContext {
        painter: ctx.painter,
        state: ctx.state,
        layout: &LayoutInfo {
          //XXX: if the element gets wrapped, this will be inaccurate.
          //But, we cant know the size of the line until we measure it, and also
          //We dont make any guarantees about this value being valid during the `measure` call
          //For all intents and purposes, this is just a *hint* for the element to use
          //(and could be just set to 0 for all we care)
          position: ctx.layout.position + line_size + line_sec_offset,
          //TODO: subtract size already taken by previous children
          max_size: self.measure_max_inner_size(ctx.layout),
          direction: self.direction,
          remaining_space: None,
        },
        current_font: ctx.current_font,
      });

      //Check the position of the side of element closest to the end on the primary axis
      let end_pos_pri = match self.direction {
        Direction::Horizontal => line_size.x + measure.size.x + padding_with_margin.sum_horizontal(),
        Direction::Vertical => line_size.y + measure.size.y + padding_with_margin.sum_vertical(),
      };

      //Wrap the element if it exceeds container's size and is not the first element in the line
      let should_wrap_overflow = self.wrap.is_enabled() && (end_pos_pri > max_line_pri);
      if self.wrap.is_allowed() && line_element_count > 0 && (measure.should_wrap || should_wrap_overflow) {
        // >>>>>>> WRAP THAT B*TCH!

        //Negate the leftover gap from the previous element
        line_size -= leftover_gap;

        //update the previous line metadata
        {
          let last_line = lines.last_mut().unwrap();
          last_line.content_size = line_size;
          //HACK: why? - self.gap, may be different for the last element or if it's the only element in the line
          let will_produce_gap = if line_element_count > 1 { self.gap } else { 0. };
          last_line.remaining_space = max_line_pri - will_produce_gap - match self.direction {
            Direction::Horizontal => line_size.x + padding_with_margin.sum_horizontal(),
            Direction::Vertical => line_size.y + padding_with_margin.sum_vertical(),
          };
        }

        //push the line metadata
        lines.push(CudLine {
          start_idx: idx,
          content_size: Vec2::ZERO,
          remaining_space: 0.,
        });

        //Update the total size accordingly
        match self.direction {
          Direction::Horizontal => {
            total_size.x = total_size.x.max(line_size.x);
            total_size.y += line_size.y + self.gap;
          },
          Direction::Vertical => {
            total_size.x += line_size.x + self.gap;
            total_size.y = total_size.y.max(line_size.y);
          }
        }

        //Now, update line_sec_offset
        match self.direction {
          Direction::Horizontal => {
            line_sec_offset.y += measure.size.y + self.gap;
          },
          Direction::Vertical => {
            line_sec_offset.x += measure.size.x + self.gap;
          }
        };

        //Reset the line size and element count
        line_size = Vec2::ZERO;
        line_element_count = 0;
      }

      //Increment element count
      line_element_count += 1;

      //Sset the leftover gap in case this is the last element in the line
      match self.direction {
        Direction::Horizontal => {
          line_size.x += measure.size.x + self.gap;
          line_size.y = line_size.y.max(measure.size.y);
          leftover_gap = vec2(self.gap, 0.);
        },
        Direction::Vertical => {
          line_size.x = line_size.x.max(measure.size.x);
          line_size.y += measure.size.y + self.gap;
          leftover_gap = vec2(0., self.gap);
        }
      }
    }

    line_size -= leftover_gap;

    //Update the content size of the last line
    {
      //HACK: why? - self.gap, may be different for the last element or if it's the only element in the line
      let cur_line = lines.last_mut().unwrap();
      cur_line.content_size = line_size;
      let will_produce_gap = if line_element_count > 1 { self.gap } else { 0. };
      cur_line.remaining_space = max_line_pri - will_produce_gap - match self.direction {
        Direction::Horizontal => line_size.x + padding_with_margin.sum_horizontal(),
        Direction::Vertical => line_size.y + padding_with_margin.sum_vertical(),
      };
    }

    //Update the total size according to the size of the last line
    match self.direction {
      Direction::Horizontal => {
        total_size.x = total_size.x.max(line_size.x);
        total_size.y += line_size.y;
      },
      Direction::Vertical => {
        total_size.x += line_size.x;
        total_size.y = total_size.y.max(line_size.y);
      }
    }

    //Now, total_size should hold the size of the AABB containing all lines
    //This is exactly what inner_content_size hint should be set to
    let inner_content_size = Some(total_size);

    //After setting the inner_content_size, we can calculate the size of the container
    //Including padding, and in case the size is set to non-auto, override the size

    total_size += vec2(
      padding_with_margin.sum_horizontal(),
      padding_with_margin.sum_vertical(),
    );

    let computed_size = compute_size(ctx.layout, self.size, total_size);
    match self.size.width {
      Size::Auto => (),
      _ => total_size.x = computed_size.x,
    }
    match self.size.height {
      Size::Auto => (),
      _ => total_size.y = computed_size.y,
    }

    // match self.size.width {
    //   Size::Auto => (),
    //   Size::Relative(percentage) => total_size.x = ctx.layout.max_size.x * percentage,
    //   Size::Absolute(pixels) => total_size.x = pixels,
    // }
    // match self.size.height {
    //   Size::Auto => (),
    //   Size::Relative(percentage) => total_size.y = ctx.layout.max_size.y * percentage,
    //   Size::Absolute(pixels) => total_size.y = pixels,
    // }

    Response {
      size: total_size,
      hints: Hints {
        inner_content_size,
        ..Default::default()
      },
      user_data: Some(Box::new(ContainerUserData { lines })),
      ..Default::default()
    }
  }

  fn process(&self, ctx: ProcessContext) {
    let user_data: &ContainerUserData = ctx.measure.user_data
      .as_ref().expect("no user data attached to container")
      .downcast_ref().expect("invalid user data type");

    let mut position = ctx.layout.position;

    let padding_with_margin = self.padding + self.margin;

    //background
    // if !self.background.is_transparent() {
    //   let corner_colors = self.background.corners();
    //   ctx.draw.add(UiDrawCommand::Rectangle {
    //     position,
    //     size: ctx.measure.size,
    //     color: corner_colors,
    //     texture: self.background_image,
    //     rounded_corners: (self.corner_radius.max_f32() > 0.).then_some({
    //       RoundedCorners::from_radius(self.corner_radius)
    //     }),
    //   });
    // }

    let frame_position = ctx.layout.position + self.margin.top_left();
    let frame_size = ctx.measure.size - vec2(padding_with_margin.sum_horizontal(), padding_with_margin.sum_vertical());
    self.background_frame.draw(ctx.paint_target, (frame_position, frame_size).into());

    //padding
    position += padding_with_margin.top_left();

    //convert alignment to pri/sec axis based
    //.0 = primary, .1 = secondary
    let pri_sec_align = match self.direction {
      Direction::Horizontal => (self.align.horizontal, self.align.vertical),
      Direction::Vertical => (self.align.vertical, self.align.horizontal),
    };

    //alignment (on sec. axis)
    // match pri_sec_align.1 {
    //   Alignment::Begin => (),
    //   Alignment::Center => {
    //     position += match self.direction {
    //       UiDirection::Horizontal => vec2(0., (ctx.measure.size.y - self.padding.top - self.padding.bottom - user_data.lines.last().unwrap().content_size.y) / 2.),
    //       UiDirection::Vertical => vec2((ctx.measure.size.x - self.padding.left - self.padding.right - user_data.lines.last().unwrap().content_size.x) / 2., 0.),
    //     };
    //   },
    //   Alignment::End => {
    //     position += match self.direction {
    //       UiDirection::Horizontal => vec2(0., ctx.measure.size.y - user_data.lines.last().unwrap().content_size.y - self.padding.bottom - self.padding.top),
    //       UiDirection::Vertical => vec2(ctx.measure.size.x - user_data.lines.last().unwrap().content_size.x - self.padding.right - self.padding.left, 0.),
    //     };
    //   }
    // }

    for (line_idx, cur_line) in user_data.lines.iter().enumerate() {
      let mut local_position = position;

      //alignment on primary axis
      match (pri_sec_align.0, self.direction) {
        (Alignment::Begin, _) => (),
        (Alignment::Center, Direction::Horizontal) => {
          local_position.x += (ctx.measure.size.x - cur_line.content_size.x) / 2. - padding_with_margin.left;
        },
        (Alignment::Center, Direction::Vertical) => {
          local_position.y += (ctx.measure.size.y - cur_line.content_size.y) / 2. - padding_with_margin.top;
        },
        (Alignment::End, Direction::Horizontal) => {
          local_position.x += ctx.measure.size.x - cur_line.content_size.x - padding_with_margin.sum_horizontal();
        },
        (Alignment::End, Direction::Vertical) => {
          local_position.y += ctx.measure.size.y - cur_line.content_size.y - padding_with_margin.sum_vertical();
        }
      }

      let next_line_begin = user_data.lines
        .get(line_idx + 1)
        .map(|l| l.start_idx)
        .unwrap_or(self.children.0.len());

      for element_idx in cur_line.start_idx..next_line_begin {
        let element = &self.children.0[element_idx];

        //(passing max size from layout rather than actual known bounds for the sake of consistency with measure() above)
        //... as this must match!

        let mut el_layout = LayoutInfo {
          position: local_position,
          max_size: self.measure_max_inner_size(ctx.layout),
          direction: self.direction,
          remaining_space: Some(cur_line.remaining_space),
        };

        //measure
        let el_measure = element.measure(MeasureContext {
          painter: ctx.painter,
          layout: &el_layout,
          state: ctx.state,
          current_font: ctx.current_font,
        });

        //align (on sec. axis)
        //TODO separate align withing the line and align of the whole line
        let inner_content_size = ctx.measure.hints.inner_content_size.unwrap();
        match (pri_sec_align.1, self.direction) {
          (Alignment::Begin, _) => (),
          (Alignment::Center, Direction::Horizontal) => {
            //Align whole row
            el_layout.position.y += ((ctx.measure.size.y - padding_with_margin.sum_vertical()) - inner_content_size.y) / 2.;
            //Align within row
            el_layout.position.y += (cur_line.content_size.y - el_measure.size.y) / 2.;
          },
          (Alignment::Center, Direction::Vertical) => {
            //Align whole row
            el_layout.position.x += ((ctx.measure.size.x - padding_with_margin.sum_horizontal()) - inner_content_size.x) / 2.;
            //Align within row
            el_layout.position.x += (cur_line.content_size.x - el_measure.size.x) / 2.;
          },
          //TODO update these two cases:
          (Alignment::End, Direction::Horizontal) => {
            //Align whole row
            el_layout.position.y += (ctx.measure.size.y - padding_with_margin.sum_vertical()) - inner_content_size.y;
            //Align within row
            el_layout.position.y += cur_line.content_size.y - el_measure.size.y;
          },
          (Alignment::End, Direction::Vertical) => {
            //Align whole row
            el_layout.position.x += (ctx.measure.size.x - padding_with_margin.sum_horizontal()) - inner_content_size.x;
            //Align within row
            el_layout.position.x += cur_line.content_size.x - el_measure.size.x;
          }
        }

        //process
        element.process(ProcessContext {
          painter: ctx.painter,
          measure: &el_measure,
          layout: &el_layout,
          paint_target: ctx.paint_target,
          state: ctx.state,
          current_font: ctx.current_font,
          input: ctx.input,
          signal: ctx.signal,
        });

        //layout
        match self.direction {
          Direction::Horizontal => {
            local_position.x += el_measure.size.x + self.gap;
          },
          Direction::Vertical => {
            local_position.y += el_measure.size.y + self.gap;
          }
        }
      }

      //Move to the next line
      match self.direction {
        Direction::Horizontal => {
          position.y += cur_line.content_size.y + self.gap;
          //position.x -= cur_line.content_size.x;
          // leftover_line_gap = vec2(0., self.gap);
        }
        Direction::Vertical => {
          position.x += cur_line.content_size.x + self.gap;
          //position.y -= cur_line.content_size.y;
          // leftover_line_gap = vec2(self.gap, 0.);
        }
      };
    }
  }
}
