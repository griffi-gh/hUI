//! Contains types which represent the sides and corners of a rectangular shape.

mod rect;
pub use rect::Rect;

mod sides;
pub use sides::Sides;

mod corners;
pub use corners::{Corners, CornersColors};
