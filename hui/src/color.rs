//! various predefined color constants

use glam::{vec4, Vec4};

/// Transparent `#00000000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00000000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const TRANSPARENT: Vec4 = vec4(0.0, 0.0, 0.0, 0.0);

/// Black `#000000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #000000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const BLACK: Vec4 = vec4(0.0, 0.0, 0.0, 1.0);

/// White `#ffffff`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ffffff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const WHITE: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);

/// Red `#ff0000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff0000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const RED: Vec4 = vec4(1.0, 0.0, 0.0, 1.0);

/// Dark red `#800000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #800000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const DARK_RED: Vec4 = vec4(0.5, 0.0, 0.0, 1.0);

/// Green `#00ff00`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00ff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const GREEN: Vec4 = vec4(0.0, 1.0, 0.0, 1.0);

/// Dark green `#008000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #008000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const DARK_GREEN: Vec4 = vec4(0.0, 0.5, 0.0, 1.0);

/// Blue `#0000ff`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #0000ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const BLUE: Vec4 = vec4(0.0, 0.0, 1.0, 1.0);

/// Dark blue `#000080`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #000080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const DARK_BLUE: Vec4 = vec4(0.0, 0.0, 0.5, 1.0);

/// Yellow `#ffff00`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ffff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const YELLOW: Vec4 = vec4(1.0, 1.0, 0.0, 1.0);

/// Cyan `#00ffff`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #00ffff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const CYAN: Vec4 = vec4(0.0, 1.0, 1.0, 1.0);

/// Magenta `#ff00ff`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff00ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const MAGENTA: Vec4 = vec4(1.0, 0.0, 1.0, 1.0);

/// Gray `#808080`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #808080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const GRAY: Vec4 = vec4(0.5, 0.5, 0.5, 1.0);

/// Light gray `#c0c0c0`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #c0c0c0; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const LIGHT_GRAY: Vec4 = vec4(0.75, 0.75, 0.75, 1.0);

/// Dark gray `#404040`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #404040; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const DARK_GRAY: Vec4 = vec4(0.25, 0.25, 0.25, 1.0);

/// Orange `#ff8000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff8000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const ORANGE: Vec4 = vec4(1.0, 0.5, 0.0, 1.0);

/// Brown `#804000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #804000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const BROWN: Vec4 = vec4(0.5, 0.25, 0.0, 1.0);

/// Pink `#ff80ff`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #ff80ff; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const PINK: Vec4 = vec4(1.0, 0.5, 1.0, 1.0);

/// Purple `#800080`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #800080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const PURPLE: Vec4 = vec4(0.5, 0.0, 0.5, 1.0);

/// Lime `#80ff00`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #80ff00; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const LIME: Vec4 = vec4(0.5, 1.0, 0.0, 1.0);

/// Teal `#008080`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #008080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const TEAL: Vec4 = vec4(0.0, 0.5, 0.5, 1.0);

/// Indigo `#004080`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #004080; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const INDIGO: Vec4 = vec4(0.0, 0.25, 0.5, 1.0);

/// Olive `#808000`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #808000; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const OLIVE: Vec4 = vec4(0.5, 0.5, 0.0, 1.0);

/// Sky blue `#87ceeb`
#[cfg_attr(doc, doc="<span style='display: inline-block; background-color: #87ceeb; width: 1em; height: 1em; border-radius: 50%; border: 1px solid black'></span>")]
pub const SKY_BLUE: Vec4 = vec4(0.53, 0.81, 0.92, 1.0);

//TODO color macro
