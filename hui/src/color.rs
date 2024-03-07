//! various predefined color constants and helper functions

use glam::{vec4, Vec4};

/// Create a color from red, green, blue components
pub fn rgb(r: u8, g: u8, b: u8) -> Vec4 {
  vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

/// Create a color from red, green, blue, alpha components
pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Vec4 {
  vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}

/// Create an RGB color from a u32 (/hex) value
pub fn rgb_hex(value: u32) -> Vec4 {
  let r = (value >> 16) & 0xff;
  let g = (value >> 8) & 0xff;
  let b = value & 0xff;
  vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

/// Create an RGBA color from a u32 (/hex) value
pub fn rgba_hex(value: u32) -> Vec4 {
  let r = (value >> 16) & 0xff;
  let g = (value >> 8) & 0xff;
  let b = value & 0xff;
  let a = (value >> 24) & 0xff;
  vec4(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a as f32 / 255.0)
}

#[cfg_attr(doc, doc="<span style='display: inline-block; background: repeating-conic-gradient(grey 0 25%,darkgrey 0 50%) 50%/8px 8px; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#00000000` Transparent
pub const TRANSPARENT: Vec4 = vec4(0.0, 0.0, 0.0, 0.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #000000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#000000` Black
pub const BLACK: Vec4 = vec4(0.0, 0.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ffffff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#ffffff` White
pub const WHITE: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff0000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#ff0000` Red
pub const RED: Vec4 = vec4(1.0, 0.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #800000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#800000` Dark red
pub const DARK_RED: Vec4 = vec4(0.5, 0.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00ff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#00ff00` Green
pub const GREEN: Vec4 = vec4(0.0, 1.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #008000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#008000` Dark green
pub const DARK_GREEN: Vec4 = vec4(0.0, 0.5, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #0000ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#0000ff` Blue
pub const BLUE: Vec4 = vec4(0.0, 0.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #000080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#000080` Dark blue
pub const DARK_BLUE: Vec4 = vec4(0.0, 0.0, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ffff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#ffff00` Yellow
pub const YELLOW: Vec4 = vec4(1.0, 1.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00ffff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#00ffff` Cyan
pub const CYAN: Vec4 = vec4(0.0, 1.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff00ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#ff00ff` Magenta
pub const MAGENTA: Vec4 = vec4(1.0, 0.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #808080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#808080` Gray
pub const GRAY: Vec4 = vec4(0.5, 0.5, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #c0c0c0; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#c0c0c0` Light gray
pub const LIGHT_GRAY: Vec4 = vec4(0.75, 0.75, 0.75, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #404040; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#404040` Dark gray
pub const DARK_GRAY: Vec4 = vec4(0.25, 0.25, 0.25, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff8000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#ff8000` Orange
pub const ORANGE: Vec4 = vec4(1.0, 0.5, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #804000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#804000` Brown
pub const BROWN: Vec4 = vec4(0.5, 0.25, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff80ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#ff80ff` Pink
pub const PINK: Vec4 = vec4(1.0, 0.5, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #800080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#800080` Purple
pub const PURPLE: Vec4 = vec4(0.5, 0.0, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #80ff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#80ff00` Lime
pub const LIME: Vec4 = vec4(0.5, 1.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #008080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#008080` Teal
pub const TEAL: Vec4 = vec4(0.0, 0.5, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #004080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#004080` Indigo
pub const INDIGO: Vec4 = vec4(0.0, 0.25, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #808000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#808000` Olive
pub const OLIVE: Vec4 = vec4(0.5, 0.5, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #87ceeb; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black; vertical-align: -7%'></span>")]
/// `#87ceeb` Sky blue
pub const SKY_BLUE: Vec4 = vec4(0.53, 0.81, 0.92, 1.0);

//TODO color macro
