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

impl TestAny for TestImplAny {
    fn greet(&self) -> String {
        format!("Hi {}", self.name)
    }
}

#[test]
fn test_as_any() {
    let instance = TestImplAny {
        name: "World".to_string(),
    };
    let trait_obj: Box<dyn TestAny> = Box::new(instance);
    // use the generated as_any helper to allow downcasting
    if let Some(inner) = trait_obj.as_any().downcast_ref::<TestImplAny>() {
        assert_eq!(inner.greet(), "Hi World");
    } else {
        panic!("Downcast failed");
    }
}
