//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use objkit_macros::as_any;

#[as_any]
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
    let animal: Box<dyn Animal> = Box::new(dog);

    println!("Animal: {}", animal.speak());

    // downcast to Dog using as_any
    if let Some(dog_ref) = animal.as_any().downcast_ref::<Dog>() {
        println!("Downcasted Dog: {}", dog_ref.speak());
    } else {
        println!("Downcast failed");
    }
}
