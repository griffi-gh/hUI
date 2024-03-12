// Layout stuff:

#[cfg(feature = "builtin_container")]
pub mod container;

#[cfg(feature = "builtin_elements")]
pub mod fill_rect;

#[cfg(feature = "builtin_elements")]
pub mod spacer;

#[cfg(feature = "builtin_elements")]
pub mod br;

// Basic elements:

#[cfg(feature = "builtin_elements")]
pub mod text;

#[cfg(feature = "builtin_elements")]
pub mod image;

// "Extras":
// (meant to be replaced if needed)

#[cfg(feature = "builtin_elements")]
pub mod progress_bar;

#[cfg(feature = "builtin_elements")]
pub mod slider;

// Wrappers:

#[cfg(feature = "builtin_elements")]
pub mod transformer;

#[cfg(feature = "builtin_elements")]
pub mod interactable;

//TODO add: Image
//TODO add: OverlayContainer (for simply laying multiple elements on top of each other)
//TODO add: Button, Checkbox, Dropdown, Input, Radio, Slider, Textarea, Toggle, etc.
//TODO add: some sort of "flexible" container (like a poor man's flexbox)
