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
use alloc::string::{String, ToString};

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
fn test_as_super() {
    let instance = TestImplSuper {
        msg: "Hello".to_string(),
    };
    // the generated helper method is named as_<traitname in lowercase>
    let trait_obj: &dyn TestSuper = instance.as_testsuper();
    assert_eq!(trait_obj.say(), "Hello");
}
