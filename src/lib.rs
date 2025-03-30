//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "README.md"))]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious
)]
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

pub use objkit_macros::as_any;
pub use objkit_macros::as_super;
pub use objkit_macros::clone_box;
pub use objkit_macros::obj_eq;
