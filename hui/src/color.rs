//! various predefined color constants

use glam::{vec4, Vec4};

#[cfg_attr(doc, doc="<span style='display: inline-block; background: repeating-conic-gradient(grey 0 25%,darkgrey 0 50%) 50%/8px 8px; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#00000000` Transparent
pub const TRANSPARENT: Vec4 = vec4(0.0, 0.0, 0.0, 0.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #000000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#000000` Black
pub const BLACK: Vec4 = vec4(0.0, 0.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ffffff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#ffffff` White
pub const WHITE: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff0000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#ff0000` Red
pub const RED: Vec4 = vec4(1.0, 0.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #800000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#800000` Dark red
pub const DARK_RED: Vec4 = vec4(0.5, 0.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00ff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#00ff00` Green
pub const GREEN: Vec4 = vec4(0.0, 1.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #008000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#008000` Dark green
pub const DARK_GREEN: Vec4 = vec4(0.0, 0.5, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #0000ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#0000ff` Blue
pub const BLUE: Vec4 = vec4(0.0, 0.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #000080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#000080` Dark blue
pub const DARK_BLUE: Vec4 = vec4(0.0, 0.0, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ffff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#ffff00` Yellow
pub const YELLOW: Vec4 = vec4(1.0, 1.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00ffff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#00ffff` Cyan
pub const CYAN: Vec4 = vec4(0.0, 1.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff00ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#ff00ff` Magenta
pub const MAGENTA: Vec4 = vec4(1.0, 0.0, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #808080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#808080` Gray
pub const GRAY: Vec4 = vec4(0.5, 0.5, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #c0c0c0; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#c0c0c0` Light gray
pub const LIGHT_GRAY: Vec4 = vec4(0.75, 0.75, 0.75, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #404040; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#404040` Dark gray
pub const DARK_GRAY: Vec4 = vec4(0.25, 0.25, 0.25, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff8000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#ff8000` Orange
pub const ORANGE: Vec4 = vec4(1.0, 0.5, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #804000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#804000` Brown
pub const BROWN: Vec4 = vec4(0.5, 0.25, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff80ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#ff80ff` Pink
pub const PINK: Vec4 = vec4(1.0, 0.5, 1.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #800080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#800080` Purple
pub const PURPLE: Vec4 = vec4(0.5, 0.0, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #80ff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#80ff00` Lime
pub const LIME: Vec4 = vec4(0.5, 1.0, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #008080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#008080` Teal
pub const TEAL: Vec4 = vec4(0.0, 0.5, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #004080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#004080` Indigo
pub const INDIGO: Vec4 = vec4(0.0, 0.25, 0.5, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #808000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#808000` Olive
pub const OLIVE: Vec4 = vec4(0.5, 0.5, 0.0, 1.0);

#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #87ceeb; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
/// `#87ceeb` Sky blue
pub const SKY_BLUE: Vec4 = vec4(0.53, 0.81, 0.92, 1.0);

//TODO color macro
