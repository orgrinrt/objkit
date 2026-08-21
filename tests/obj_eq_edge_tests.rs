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

use objkit::obj_eq;

#[obj_eq]
pub trait TestEq {
    fn value(&self) -> i32;
}

#[derive(PartialEq)]
struct TestImplEq {
    val: i32,
}

impl TestEq for TestImplEq {
    fn value(&self) -> i32 {
        self.val
    }
}

#[test]
fn test_obj_eq_trait_objects() {
    let a = TestImplEq {
        val: 50,
    };
    let b = TestImplEq {
        val: 50,
    };
    let c = TestImplEq {
        val: 75,
    };

    let a_obj = &a as &dyn TestEq;
    let b_obj = &b as &dyn TestEq;
    let c_obj = &c as &dyn TestEq;

    // compare trait objects
    assert!(a_obj == b_obj);
    assert!(a_obj != c_obj);
    // verify symmetry using the generated .eq method
    assert!(a_obj.eq(&b_obj));
    assert!(!a_obj.eq(&c_obj));
}

#[test]
fn test_obj_eq_boxes_independence() {
    let a = TestImplEq {
        val: 200,
    };
    let b = TestImplEq {
        val: 200,
    };

    let a_box = Box::new(a) as Box<dyn TestEq>;
    let b_box = Box::new(b) as Box<dyn TestEq>;

    // boxing creates independent objects even if values are equal
    assert!(a_box == b_box);
}

#[test]
fn test_obj_eq_boxes_independence_eq_call() {
    let a = TestImplEq {
        val: 200,
    };
    let b = TestImplEq {
        val: 200,
    };

    let a_box = Box::new(a) as Box<dyn TestEq>;
    let b_box = Box::new(b) as Box<dyn TestEq>;

    // check that the equality method works consistently.
    assert!(a_box.eq(&b_box));
}
