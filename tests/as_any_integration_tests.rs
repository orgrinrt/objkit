//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
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
