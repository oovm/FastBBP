#![allow(non_snake_case)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

extern crate alloc;

mod errors;
mod order_1;
mod order_2;
mod order_3;
mod order_4294967296;

mod helpers;

pub use crate::{
    errors::{EvaluateError, Result},
    order_1::{chudnovsky, RamanujanL1},
    order_2::{ramanujan, RamanujanL2},
    order_3::{bbp65536, RamanujanL3},
};
