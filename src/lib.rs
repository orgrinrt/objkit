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

// Always, not only under `no_std`. `alloc` is present on a std target too, and
// declaring it unconditionally is what lets `__objkit_box` below be one path
// rather than a feature-selected pair.
extern crate alloc;

// Where every expansion reaches `Box`.
//
// Re-exported rather than named at the expansion site, for the same reason the
// hasher is: the expansion lands in the consumer's crate, and
// `::alloc::boxed::Box` resolves there only if that consumer declared `extern
// crate alloc`, which a plain `std` crate has no reason to have done, while
// `::std::boxed::Box` resolves only if it is not `#![no_std]`.
//
// Reached through this crate it resolves wherever this crate does, so one
// expansion serves both kinds of consumer and the choice is not a feature. A
// feature here would have read as additive and would not be: cargo unifies
// features across a dependency graph, so one sibling picking the `alloc` path
// would change what every unrelated consumer's macros emit.
#[doc(hidden)]
pub use alloc::boxed::Box as __objkit_box;

// Where the `obj_eq` expansion reaches its hasher under `no_std`.
//
// Re-exported rather than named at the expansion site, because the expansion lands in
// the consumer's crate, where `::siphasher` resolves only if that consumer happens to
// depend on it under that name, which nothing tells it to do. Reached through this crate
// it resolves wherever this crate does.
//
// The type id needs no crate: `core::any::TypeId` has been there since 1.0. The
// expansion used to name `::typeable::TypeId`, which never resolved at all, because that
// item is a private re-import of `std::any::TypeId` inside a crate that is itself `std`.
#[cfg(feature = "no_std")]
#[doc(hidden)]
pub use ::siphasher as __objkit_siphasher;
// A boxed clone is exactly what an allocator is for, so there is nothing to offer
// without one. Absent rather than present-and-failing, so a consumer finds out at the
// import rather than inside an expansion.
#[cfg(not(feature = "no_alloc"))]
pub use objkit_macros::clone_box;
/// Compares two trait objects of a trait annotated with [`macro@obj_eq`].
///
/// Without the attribute there is no `PartialEq` for the trait object, and
/// comparing two of them does not compile:
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
/// let a = Counted {
///     val: 100,
/// };
/// let b = Counted {
///     val: 100,
/// };
/// let c = Counted {
///     val: 7,
/// };
///
/// assert!((&a as &dyn Value) == (&b as &dyn Value));
/// assert!((&a as &dyn Value) != (&c as &dyn Value));
/// ```
pub use objkit_macros::obj_eq;
pub use objkit_macros::{as_any, as_super};
