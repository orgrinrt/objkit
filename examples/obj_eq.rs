//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use objkit_macros::obj_eq;

#[obj_eq]
pub trait Animal {
    fn speak(&self) -> String;
}

#[derive(PartialEq)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

#[derive(PartialEq)]
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

fn main() {
    let dog1 = Dog {
        name: "Buddy".to_string(),
    };
    let dog2 = Dog {
        name: "Buddy".to_string(),
    };
    let cat = Cat {
        name: "Whiskers".to_string(),
    };

    let animal_dog1 = &dog1 as &dyn Animal;
    let animal_dog2 = &dog2 as &dyn Animal;
    let animal_cat = &cat as &dyn Animal;

    println!("dog1 eq dog2: {}", animal_dog1 == animal_dog2);
    println!("dog1 eq cat: {}", animal_dog1.eq(&animal_cat));
    println!("dog1 speaks: {}", animal_dog1.speak());
    println!("cat speaks: {}", animal_cat.speak());
}
