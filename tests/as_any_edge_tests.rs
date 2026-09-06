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

use objkit::as_any;

#[as_any]
pub trait TestAny {
    fn greet(&self) -> String;
}

struct TestImplAny {
    name: String,
}

struct WrongType;

impl TestAny for TestImplAny {
    fn greet(&self) -> String {
        format!("Hello {}", self.name)
    }
}

#[test]
fn test_as_any_downcast_success() {
    let instance = TestImplAny {
        name: "Alice".to_string(),
    };
    let boxed: Box<dyn TestAny> = Box::new(instance);
    if let Some(inner) = boxed.as_any().downcast_ref::<TestImplAny>() {
        assert_eq!(inner.greet(), "Hello Alice");
    } else {
        panic!("Expected a valid downcast.");
    }
}

#[test]
fn test_as_any_downcast_failure() {
    let instance = TestImplAny {
        name: "Bob".to_string(),
    };
    let boxed: Box<dyn TestAny> = Box::new(instance);
    // attempt to downcast to a wrong type returns None
    assert!(boxed.as_any().downcast_ref::<WrongType>().is_none());
}
