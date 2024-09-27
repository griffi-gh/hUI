#![doc(html_logo_url = "https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/hui.svg")]
//!
//! Simple UI library for games and other interactive applications
//!
//! # Features
#![doc = document_features::document_features!()]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#![deny(unsafe_code)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(unused_parens)]

// Re-export hui-painter
pub use hui_painter as painter;

pub use hui_shared::*;

mod instance;
mod macros;
pub mod layout;
pub mod element;
pub mod event;
pub mod input;
pub mod measure;
pub mod state;
pub mod signal;
pub mod frame;

pub use instance::UiInstance;
