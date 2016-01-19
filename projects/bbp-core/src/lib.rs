#![allow(non_snake_case)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]


mod errors;
mod order_16;
mod order_256;
mod helpers;

pub use crate::errors::{EvaluateError, Result};
pub use crate::order_16::{bbp16, PiViewerBase16};
pub use crate::order_256::{bbp256, PiViewerBase256};