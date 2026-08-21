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
// `clippy::cargo` wants a positive feature name, and it is right in general: a negative one
// cannot be additive, which is what cargo features are. `no_std` stays because it is the
// name the ecosystem uses for exactly this and because it is already published, so renaming
// it breaks every consumer to satisfy a lint. The refusals in `objkit-macros` say what to
// do when both or neither is selected, which is the failure the lint is really about.
#![allow(clippy::negative_feature_names)]
#![cfg_attr(feature = "no_std", no_std)]


#[cfg(feature = "no_std")]
extern crate alloc;

pub use objkit_macros::as_any;
pub use objkit_macros::as_super;
pub use objkit_macros::clone_box;
/// Compares two trait objects of a trait annotated with [`macro@obj_eq`].
///
/// Without the attribute there is no `PartialEq` for the trait object, and comparing two
/// of them does not compile:
///
/// ```compile_fail,E0369
/// pub trait Value {
///     fn value(&self) -> i32;
/// }
///
/// #[derive(PartialEq)]
/// struct Counted {
///     val: i32,
/// }
///
/// impl Value for Counted {
///     fn value(&self) -> i32 {
///         self.val
///     }
/// }
///
/// let a = Counted { val: 100 };
/// let b = Counted { val: 100 };
///
/// // `==` cannot be applied to `&dyn Value`: E0369.
/// let _ = (&a as &dyn Value) == (&b as &dyn Value);
/// ```
///
/// With it, the same comparison resolves through the generated auxiliary trait:
///
/// ```
/// use objkit::obj_eq;
///
/// #[obj_eq]
/// pub trait Value {
///     fn value(&self) -> i32;
/// }
///
/// #[derive(PartialEq)]
/// struct Counted {
///     val: i32,
/// }
///
/// impl Value for Counted {
///     fn value(&self) -> i32 {
///         self.val
///     }
/// }
///
/// let a = Counted { val: 100 };
/// let b = Counted { val: 100 };
/// let c = Counted { val: 7 };
///
/// assert!((&a as &dyn Value) == (&b as &dyn Value));
/// assert!((&a as &dyn Value) != (&c as &dyn Value));
/// ```
pub use objkit_macros::obj_eq;
