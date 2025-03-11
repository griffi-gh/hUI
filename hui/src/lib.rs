#![no_std]
#![doc(html_logo_url = "https://raw.githubusercontent.com/griffi-gh/hui/master/.assets/hui.svg")]
//!
//! Simple UI library for games and other interactive applications
//!
//! # Features
#![doc = document_features::document_features!()]

#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#![forbid(unsafe_op_in_unsafe_fn)]
#![allow(unused_parens)]

#[macro_use]
extern crate alloc;

pub use hui_shared::*;
pub use hui_painter as painter;

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
pub mod font;

pub use instance::UiInstance;
