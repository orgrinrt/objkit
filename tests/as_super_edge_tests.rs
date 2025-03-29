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
fn test_as_super_reference_and_box() {
    let instance = TestImplSuper {
        msg: "Edge".to_string(),
    };

    // using as_super on a reference and verifying that the returned reference is the same
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
