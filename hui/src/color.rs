use glam::{vec4, Vec4};

/// Transparent `#00000000`
pub const TRANSPARENT: Vec4 = vec4(0.0, 0.0, 0.0, 0.0);

/// Black `#000000`
pub const BLACK: Vec4 = vec4(0.0, 0.0, 0.0, 1.0);

/// White `#ffffff`
pub const WHITE: Vec4 = vec4(1.0, 1.0, 1.0, 1.0);

/// Red `#ff0000`
pub const RED: Vec4 = vec4(1.0, 0.0, 0.0, 1.0);

/// Dark red `#800000`
pub const DARK_RED: Vec4 = vec4(0.5, 0.0, 0.0, 1.0);

/// Green `#00ff00`
pub const GREEN: Vec4 = vec4(0.0, 1.0, 0.0, 1.0);

/// Dark green `#008000`
pub const DARK_GREEN: Vec4 = vec4(0.0, 0.5, 0.0, 1.0);

/// Blue `#0000ff`
pub const BLUE: Vec4 = vec4(0.0, 0.0, 1.0, 1.0);

/// Dark blue `#000080`
pub const DARK_BLUE: Vec4 = vec4(0.0, 0.0, 0.5, 1.0);

/// Yellow `#ffff00`
pub const YELLOW: Vec4 = vec4(1.0, 1.0, 0.0, 1.0);

/// Cyan `#00ffff`
pub const CYAN: Vec4 = vec4(0.0, 1.0, 1.0, 1.0);

/// Magenta `#ff00ff`
pub const MAGENTA: Vec4 = vec4(1.0, 0.0, 1.0, 1.0);

/// Gray `#808080`
pub const GRAY: Vec4 = vec4(0.5, 0.5, 0.5, 1.0);

/// Light gray `#c0c0c0`
pub const LIGHT_GRAY: Vec4 = vec4(0.75, 0.75, 0.75, 1.0);

/// Dark gray `#404040`
pub const DARK_GRAY: Vec4 = vec4(0.25, 0.25, 0.25, 1.0);

/// Orange `#ff8000`
pub const ORANGE: Vec4 = vec4(1.0, 0.5, 0.0, 1.0);

/// Brown `#804000`
pub const BROWN: Vec4 = vec4(0.5, 0.25, 0.0, 1.0);

/// Pink `#ff80ff`
pub const PINK: Vec4 = vec4(1.0, 0.5, 1.0, 1.0);

/// Purple `#800080`
pub const PURPLE: Vec4 = vec4(0.5, 0.0, 0.5, 1.0);

/// Lime `#80ff00`
pub const LIME: Vec4 = vec4(0.5, 1.0, 0.0, 1.0);

/// Teal `#008080`
pub const TEAL: Vec4 = vec4(0.0, 0.5, 0.5, 1.0);

/// Indigo `#004080`
pub const INDIGO: Vec4 = vec4(0.0, 0.25, 0.5, 1.0);

/// Olive `#808000`
pub const OLIVE: Vec4 = vec4(0.5, 0.5, 0.0, 1.0);

/// Sky blue `#87ceeb`
pub const SKY_BLUE: Vec4 = vec4(0.53, 0.81, 0.92, 1.0);

//TODO color macro
