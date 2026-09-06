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

// Two implementors with the same layout and the same payload. Nothing about the
// bytes tells them apart, so this is the case the type check alone answers, and
// it is the case an equality that compared representations would get wrong.
#[derive(PartialEq)]
struct Metres(i32);

#[derive(PartialEq)]
struct Seconds(i32);

impl TestEq for Metres {
    fn value(&self) -> i32 {
        self.0
    }
}

impl TestEq for Seconds {
    fn value(&self) -> i32 {
        self.0
    }
}

#[test]
fn same_layout_and_payload_behind_different_types_is_not_equal() {
    let m = Metres(5);
    let s = Seconds(5);
    assert_eq!(
        m.value(),
        s.value(),
        "the payloads agree, which is the point of the case"
    );
    assert!((&m as &dyn TestEq) != (&s as &dyn TestEq));
    assert!((&s as &dyn TestEq) != (&m as &dyn TestEq));
}

#[cfg(not(feature = "no_alloc"))]
#[test]
fn same_layout_and_payload_behind_different_boxed_types_is_not_equal() {
    let boxed_m: Box<dyn TestEq> = Box::new(Metres(5));
    let boxed_s: Box<dyn TestEq> = Box::new(Seconds(5));
    assert!(boxed_m != boxed_s);
}

// An implementor whose `PartialEq` is not the structural one. The comparison
// through the trait object has to be that implementation and not a byte
// comparison, or a type that treats two representations as one value would be
// told they differ.
struct Modulo10(i32);

impl PartialEq for Modulo10 {
    fn eq(&self, other: &Self) -> bool {
        self.0.rem_euclid(10) == other.0.rem_euclid(10)
    }
}

impl TestEq for Modulo10 {
    fn value(&self) -> i32 {
        self.0
    }
}

#[test]
fn the_implementor_s_own_partial_eq_is_what_runs() {
    let a = Modulo10(3);
    let b = Modulo10(13);
    let c = Modulo10(4);
    assert!(
        (&a as &dyn TestEq) == (&b as &dyn TestEq),
        "3 and 13 are one value mod 10"
    );
    assert!((&a as &dyn TestEq) != (&c as &dyn TestEq));
}

#[test]
fn a_value_compares_equal_to_itself_through_the_object() {
    let a = TestImplEq {
        val: 9,
    };
    let object = &a as &dyn TestEq;
    assert!(object == object);
}
