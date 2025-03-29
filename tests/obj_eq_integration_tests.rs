//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
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
fn test_obj_eq() {
    let a = TestImplEq {
        val: 100,
    };
    let b = TestImplEq {
        val: 100,
    };
    let c = TestImplEq {
        val: 200,
    };

    let a_obj = &a as &dyn TestEq;
    let b_obj = &b as &dyn TestEq;
    let c_obj = &c as &dyn TestEq;

    // the generated implementation makes equality comparisons work on trait objects

    assert!(a_obj == b_obj);
    assert!(a_obj != c_obj);

    assert!(a_obj.eq(&b_obj));
    assert!(!a_obj.eq(&c_obj));
}

#[test]
fn test_obj_eq_boxes() {
    let a = TestImplEq {
        val: 100,
    };
    let b = TestImplEq {
        val: 100,
    };
    let c = TestImplEq {
        val: 200,
    };

    let a_box = Box::new(a) as Box<dyn TestEq>;
    let b_box = Box::new(b) as Box<dyn TestEq>;
    let c_box = Box::new(c) as Box<dyn TestEq>;

    // verify object equality on boxed trait objects
    assert!(a_box == b_box);
    assert!(a_box != c_box);
}

#[test]
fn test_obj_eq_boxes_eq_call() {
    let a = TestImplEq {
        val: 100,
    };
    let b = TestImplEq {
        val: 100,
    };
    let c = TestImplEq {
        val: 200,
    };

    let a_box = Box::new(a) as Box<dyn TestEq>;
    let b_box = Box::new(b) as Box<dyn TestEq>;
    let c_box = Box::new(c) as Box<dyn TestEq>;

    // verify object equality on boxed trait objects
    assert!(a_box.eq(&b_box));
    assert!(!a_box.eq(&c_box));
}

/// ```compile_fail
/// // This test verifies that without the attribute macro, the trait object
/// // does not support the equality operator.
///
/// // A trait without the attribute macro
/// pub trait TestEq {
///     fn value(&self) -> i32;
/// }
///
/// #[derive(PartialEq)]
/// struct TestImpl {
///     val: i32,
/// }
///
/// impl TestEq for TestImpl {
///     fn value(&self) -> i32 {
///         self.val
///     }
/// }
///
/// fn main() {
///     let a = TestImpl { val: 100 };
///     let b = TestImpl { val: 100 };
///
///     // Convert to trait objects
///     let a_obj = &a as &dyn TestEq;
///     let b_obj = &b as &dyn TestEq;
///
///     // This line is expected to fail to compile because there is no PartialEq
///     // implementation for &dyn TestEq
///     assert_eq!(a_obj, b_obj);
/// }
/// ```
///
pub fn dummy() {}
