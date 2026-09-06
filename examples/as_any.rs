//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

use objkit::as_any;

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

    // The refusal, which is the half worth seeing: a downcast to the wrong concrete
    // type answers `None` rather than doing something. With only the succeeding
    // case, the example shows that `as_any` returns something and not that it
    // checks anything.
    if animal.as_any().downcast_ref::<Cat>().is_some() {
        println!("Downcast to Cat: unreachable, this animal is a Dog");
    } else {
        println!("Downcast to Cat refused, which is what makes the one above mean something");
    }

    let cat: Box<dyn Animal> = Box::new(Cat {
        name: "Momo".to_string(),
    });
    if let Some(cat_ref) = cat.as_any().downcast_ref::<Cat>() {
        println!("Downcasted Cat: {}", cat_ref.speak());
    }
}
