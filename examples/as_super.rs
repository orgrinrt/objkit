//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use objkit_macros::as_super;

#[as_super]
pub trait Animal {
    fn speak(&self) -> String;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

fn main() {
    let dog = Dog {
        name: "Buddy".to_string(),
    };

    println!("Dog speaks: {}", dog.speak());

    let animal_super: &dyn Animal = dog.as_animal();
    println!("Super Animal speaks: {}", animal_super.speak());
}
