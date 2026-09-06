//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

// The `no_std` prelude has neither, so declaring `alloc` is only half of it:
// the names have to be brought in too. Without this the crate builds under
// `no_std` and its tests do not, which is the shape that hides: `cargo build`
// is green and only `cargo test` says otherwise.
#[cfg(feature = "no_std")]
use alloc::boxed::Box;

use objkit::clone_box;

#[clone_box]
pub trait TestTrait {
    fn value(&self) -> i32;
}

#[derive(Clone)]
struct TestImpl {
    val: i32,
}

impl TestTrait for TestImpl {
    fn value(&self) -> i32 {
        self.val
    }
}

#[test]
fn test_clone_box() {
    let original = Box::new(TestImpl {
        val: 42,
    }) as Box<dyn TestTrait>;
    let cloned = original.clone();

    assert_eq!(original.value(), 42);
    assert_eq!(cloned.value(), 42);

    // verify they are different objects
    //
    // NOTE: the below compares the fat ptrs which can lead to false positives
    // let original_ptr = &*original as *const dyn TestTrait;
    // let cloned_ptr = &*cloned as *const dyn TestTrait;
    //
    // NOTE: so we compare the data pointers instead below
    //       (leaving these notices here for future reference)
    let original_ptr = (&*original as *const dyn TestTrait) as *const ();
    let cloned_ptr = (&*cloned as *const dyn TestTrait) as *const ();
    assert_ne!(original_ptr, cloned_ptr);
}

#[test]
fn test_direct_clone_box() {
    let original = Box::new(TestImpl {
        val: 42,
    }) as Box<dyn TestTrait>;
    let cloned = original.clone_box();

    assert_eq!(original.value(), 42);
    assert_eq!(cloned.value(), 42);
}
