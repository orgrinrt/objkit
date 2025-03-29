//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
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
