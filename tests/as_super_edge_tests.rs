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
#[cfg(feature = "no_std")]
use alloc::format;
#[cfg(feature = "no_std")]
use alloc::string::{String, ToString};
#[cfg(feature = "no_std")]
use alloc::vec::Vec;

use objkit::as_super;

#[as_super]
pub trait TestSuper {
    fn say(&self) -> String;
}

struct TestImplSuper {
    msg: String,
}

impl TestSuper for TestImplSuper {
    fn say(&self) -> String {
        self.msg.clone()
    }
}

#[test]
fn test_as_super_reference_and_box() {
    let instance = TestImplSuper {
        msg: "Edge".to_string(),
    };

    // using as_super on a reference and verifying that the returned reference is
    // the same
    let trait_ref: &dyn TestSuper = instance.as_testsuper();
    assert_eq!(trait_ref.say(), "Edge");

    // test that boxing does not break the as_super conversion
    let boxed_instance = Box::new(TestImplSuper {
        msg: "Boxed Edge".to_string(),
    });
    let trait_box: &dyn TestSuper = boxed_instance.as_testsuper();
    assert_eq!(trait_box.say(), "Boxed Edge");

    // check that repeated calls return the same pointer (w/ fat ptrs)
    let ptr1 = trait_ref as *const dyn TestSuper as *const () as usize;
    let ptr2 = instance.as_testsuper() as *const dyn TestSuper as *const () as usize;
    assert_eq!(ptr1, ptr2);
}
