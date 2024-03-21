//! contains types which represent the sides and corners of a rectangular shape.

//XXX: this is kinda a mess, either move the rect struct here or come up with a better name for this module
#[allow(clippy::module_inception)]
mod rect;
pub use rect::Rect;

mod sides;
pub use sides::Sides;

mod corners;
pub use corners::Corners;

mod color;
pub use color::FillColor;
