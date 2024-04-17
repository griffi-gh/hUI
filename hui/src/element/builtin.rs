// Layout stuff:

#[cfg(feature = "el_container")]
pub mod container;

#[cfg(feature = "el_frame_view")]
pub mod frame_view;

#[cfg(feature = "el_spacer")]
pub mod spacer;

#[cfg(feature = "el_br")]
pub mod br;

// Basic elements:

#[cfg(feature = "el_text")]
pub mod text;

#[cfg(feature = "el_image")]
pub mod image;

// "Extras":
// (meant to be replaced if needed)

#[cfg(feature = "el_progress_bar")]
pub mod progress_bar;

#[cfg(feature = "el_slider")]
pub mod slider;

// Wrappers:

#[cfg(feature = "el_transformer")]
pub mod transformer;

#[cfg(feature = "el_interactable")]
pub mod interactable;

//TODO add: Image
//TODO add: OverlayContainer (for simply laying multiple elements on top of each other)
//TODO add: Button, Checkbox, Dropdown, Input, Radio, Slider, Textarea, Toggle, etc.
//TODO add: some sort of "flexible" container (like a poor man's flexbox)
