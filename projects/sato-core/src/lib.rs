#![allow(non_snake_case)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate alloc;

mod errors;
mod order_1;
mod order_256;
mod order_4294967296;
mod order_65536;

mod helpers;

pub use crate::{
    errors::{EvaluateError, Result},
    order_256::{bbp256, PiViewer8},
    order_65536::{bbp65536, PiViewer16},
};
