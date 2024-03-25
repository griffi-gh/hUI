use glam::{vec2, UVec2, Vec2, Vec4};
use crate::{color, draw::{ImageHandle, UiDrawCommand}, rect::{Corners, FillColor, Rect}};
use super::Frame;

#[derive(Clone, Copy, Debug)]
pub struct NinePatchAsset {
  pub image: ImageHandle,
  //TODO: remove this
  pub size: (u32, u32),
  pub scalable_region: Rect,
}

//TODO allow scaling/moving corners
#[derive(Clone, Copy, Debug)]
pub struct NinePatchFrame {
  pub asset: NinePatchAsset,
  pub color: FillColor,
}

impl NinePatchFrame {
  pub fn from_asset(asset: NinePatchAsset) -> Self {
    Self { asset, ..Default::default() }
  }

  pub fn with_color(mut self, color: impl Into<FillColor>) -> Self {
    self.color = color.into();
    self
  }
}

impl Default for NinePatchFrame {
  fn default() -> Self {
    Self {
      //This is not supposed to be left out as the default, so just set it to whatever :p
      asset: NinePatchAsset { image: ImageHandle::default(), size: (0, 0), scalable_region: Rect::default() },
      color: color::WHITE.into(),
    }
  }
}

impl Frame for NinePatchFrame {
  fn draw(&self, draw: &mut crate::draw::UiDrawCommandList, position: glam::Vec2, parent_size: glam::Vec2) {
    //FIXME: this algrorightm shїts itself when parent_size < img_sz

    // without this, shїt gets messed up when the position is not a whole number
    //XXX: should we round the size as well?
    let position = position.round();

    let img_sz = UVec2::from(self.asset.size).as_vec2();

    //Color stuff
    let interpolate_color_rect = |uvs: Corners<Vec2>| {
      Corners {
        top_left: self.color.interpolate(uvs.top_left),
        top_right: self.color.interpolate(uvs.top_right),
        bottom_left: self.color.interpolate(uvs.bottom_left),
        bottom_right: self.color.interpolate(uvs.bottom_right),
      }
    };

    // Inset coords, in UV space
    let region_uv = self.asset.scalable_region.corners();

    // Inset coords, in image (px) space
    let corners_image_px = Corners {
      top_left: img_sz * region_uv.top_left,
      top_right: img_sz * region_uv.top_right,
      bottom_left: img_sz * region_uv.bottom_left,
      bottom_right: img_sz * region_uv.bottom_right,
    };

    let size_h = (
      corners_image_px.top_left.x,
      parent_size.x - corners_image_px.top_left.x - (img_sz.x - corners_image_px.top_right.x),
      img_sz.x - corners_image_px.top_right.x,
    );

    let size_v = (
      corners_image_px.top_left.y,
      parent_size.y - corners_image_px.top_left.y - (img_sz.y - corners_image_px.bottom_left.y),
      img_sz.y - corners_image_px.bottom_left.y,
    );

    //Top-left patch
    let top_left_patch_uv = Corners {
      top_left: vec2(0., 0.),
      top_right: vec2(region_uv.top_left.x, 0.),
      bottom_left: vec2(0., region_uv.top_left.y),
      bottom_right: region_uv.top_left,
    };
    draw.add(UiDrawCommand::Rectangle {
      position,
      size: vec2(size_h.0, size_v.0),
      color: interpolate_color_rect(top_left_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(top_left_patch_uv),
      rounded_corners: None
    });

    //Top patch
    let top_patch_uv = Corners {
      top_left: vec2(region_uv.top_left.x, 0.),
      top_right: vec2(region_uv.top_right.x, 0.),
      bottom_left: region_uv.top_left,
      bottom_right: region_uv.top_right,
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(size_h.0, 0.),
      size: vec2(size_h.1, size_v.0),
      color: interpolate_color_rect(top_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(top_patch_uv),
      rounded_corners: None
    });

    //Top-right patch
    let top_right_patch_uv = Corners {
      top_left: vec2(region_uv.top_right.x, 0.),
      top_right: vec2(1., 0.),
      bottom_left: region_uv.top_right,
      bottom_right: vec2(1., region_uv.top_right.y),
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(size_h.0 + size_h.1, 0.),
      size: vec2(size_h.2, size_v.0),
      color: interpolate_color_rect(top_right_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(top_right_patch_uv),
      rounded_corners: None
    });

    //Left patch
    let left_patch_uv = Corners {
      top_left: vec2(0., region_uv.top_left.y),
      top_right: region_uv.top_left,
      bottom_left: vec2(0., region_uv.bottom_left.y),
      bottom_right: region_uv.bottom_left,
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(0., size_v.0),
      size: vec2(size_h.0, size_v.1),
      color: interpolate_color_rect(left_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(left_patch_uv),
      rounded_corners: None
    });

    // Center patch
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(size_h.0, size_v.0),
      size: vec2(size_h.1, size_v.1),
      color: interpolate_color_rect(region_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(region_uv),
      rounded_corners: None
    });

    //Right patch
    let right_patch_uv = Corners {
      top_left: region_uv.top_right,
      top_right: vec2(1., region_uv.top_right.y),
      bottom_left: region_uv.bottom_right,
      bottom_right: vec2(1., region_uv.bottom_right.y),
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(size_h.0 + size_h.1, size_v.0),
      size: vec2(size_h.2, size_v.1),
      color: interpolate_color_rect(right_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(right_patch_uv),
      rounded_corners: None
    });

    //Bottom-left patch
    let bottom_left_patch_uv = Corners {
      top_left: vec2(0., region_uv.bottom_left.y),
      top_right: region_uv.bottom_left,
      bottom_left: vec2(0., 1.),
      bottom_right: vec2(region_uv.bottom_left.x, 1.),
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(0., size_v.0 + size_v.1),
      size: vec2(size_h.0, size_v.2),
      color: interpolate_color_rect(bottom_left_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(bottom_left_patch_uv),
      rounded_corners: None
    });

    //Bottom patch
    let bottom_patch_uv = Corners {
      top_left: region_uv.bottom_left,
      top_right: region_uv.bottom_right,
      bottom_left: vec2(region_uv.bottom_left.x, 1.),
      bottom_right: vec2(region_uv.bottom_right.x, 1.),
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(size_h.0, size_v.0 + size_v.1),
      size: vec2(size_h.1, size_v.2),
      color: interpolate_color_rect(bottom_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(bottom_patch_uv),
      rounded_corners: None
    });

    //Bottom-right patch
    let bottom_right_patch_uv = Corners {
      top_left: region_uv.bottom_right,
      top_right: vec2(1., region_uv.bottom_right.y),
      bottom_left: vec2(region_uv.bottom_right.x, 1.),
      bottom_right: vec2(1., 1.),
    };
    draw.add(UiDrawCommand::Rectangle {
      position: position + vec2(size_h.0 + size_h.1, size_v.0 + size_v.1),
      size: vec2(size_h.2, size_v.2),
      color: interpolate_color_rect(bottom_right_patch_uv),
      texture: Some(self.asset.image),
      texture_uv: Some(bottom_right_patch_uv),
      rounded_corners: None
    });
  }

  fn covers_opaque(&self) -> bool {
    false
  }
}
