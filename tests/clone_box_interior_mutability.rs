//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use objkit::clone_box;
use std::any::Any;
use std::cell::Cell;

#[clone_box]
pub trait AdvancedTrait: Any {
    fn value(&self) -> i32;
    fn as_any(&self) -> &dyn Any;
}

#[derive(Clone)]
struct ValidAdvancedImpl {
    val: i32,
    counter: Cell<u32>,
}

impl AdvancedTrait for ValidAdvancedImpl {
    fn value(&self) -> i32 {
        self.val
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn test_valid_clone_box() {
    let original = Box::new(ValidAdvancedImpl {
        val: 100,
        counter: Cell::new(0),
    }) as Box<dyn AdvancedTrait>;
    let cloned = original.clone();

    let orig_impl = original
        .as_any()
        .downcast_ref::<ValidAdvancedImpl>()
        .unwrap();
    let clone_impl = cloned.as_any().downcast_ref::<ValidAdvancedImpl>().unwrap();

    orig_impl.counter.set(5);
    assert_eq!(orig_impl.counter.get(), 5);
    assert_eq!(clone_impl.counter.get(), 0);
}
