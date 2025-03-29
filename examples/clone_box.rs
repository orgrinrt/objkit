//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use objkit_macros::clone_box;

#[clone_box]
pub trait Animal {
    fn speak(&self) -> String;
}

#[derive(Clone)]
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn speak(&self) -> String {
        format!("{} says: Woof!", self.name)
    }
}

#[derive(Clone)]
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn speak(&self) -> String {
        format!("{} says: Meow!", self.name)
    }
}

fn main() {
    // create some animal instances
    let dog = Dog {
        name: "Buddy".to_string(),
    };
    let cat = Cat {
        name: "Whiskers".to_string(),
    };

    // create trait objects
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(dog), Box::new(cat)];

    // clone the vector of trait objects
    let cloned_animals = animals.clone();

    // demonstrate that we have two separate collections
    for animal in &animals {
        println!("Original: {}", animal.speak());
    }

    for animal in &cloned_animals {
        println!("Cloned: {}", animal.speak());
    }
}
