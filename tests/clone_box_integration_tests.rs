//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use traitkit::clone_box;

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
    let original_ptr = &*original as *const dyn TestTrait;
    let cloned_ptr = &*cloned as *const dyn TestTrait;
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
