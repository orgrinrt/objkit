# traitkit

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/traitkit.svg)](https://github.com/orgrinrt/traitkit/stargazers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/traitkit)](https://crates.io/crates/traitkit)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/traitkit.svg)](https://github.com/orgrinrt/traitkit/issues)
[![Latest Version](https://img.shields.io/badge/version-2.0.6-blue.svg?label=latest)](https://github.com/orgrinrt/traitkit)
![Crates.io Version](https://img.shields.io/crates/v/traitkit?logoSize=auto&color=%23FDC700&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftraitkit)
![Crates.io Size](https://img.shields.io/crates/size/traitkit?color=%23C27AFF&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftraitkit)
![Crates.io MSRV](https://img.shields.io/crates/msrv/traitkit?style=flat&logoSize=auto&color=%23D08700&link=https%3A%2F%2Fcrates.io%2Fcrates%2Ftraitkit)
![GitHub last commit](https://img.shields.io/github/last-commit/orgrinrt/traitkit?color=%23009689&link=https%3A%2F%2Fgithub.com%2Forgrinrt%2Ftraitkit)

> A toolkit providing minimal-cost abstractions of well-established patterns for trait object operations that aren't supported by rust's trait system directly, such as cloning, comparison, and conversion

</div>

## Features

| Feature     | Status      | Description         |
|-------------|-------------|---------------------|
| `clone_box` | ✅ Available | `clone_box` pattern |
| comparison  | 🚧 Planned  |                     |
| conversion  | 🚧 Planned  |                     |

## Usage

This crate provides procedural macros that enhance rust traits by enabling operations that aren't natively supported for trait objects. Currently, the sole feature is the
`clone_box` attribute, which enables cloning of trait objects with minimal abstraction overhead beyond the unavoidable dynamic dispatch.

```rust
use traitkit::clone_box;

#[clone_box]
pub trait MyTrait {}
```

## Example

Here's a simple example showing how to use the `clone_box` attribute to create clonable trait objects without the boilerplate:

```rust
use traitkit::clone_box;

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
    // create a vector of trait objects
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog { name: "Buddy".to_string() }),
        Box::new(Cat { name: "Whiskers".to_string() }),
    ];

    // clone the vector of trait objects 
    // (this is where the clone_box pattern comes in handy)
    let cloned_animals = animals.clone();

    // both vecs work as you'd expect
    for animal in &animals {
        println!("Original: {}", animal.speak());
    }

    for animal in &cloned_animals {
        println!("Cloned: {}", animal.speak());
    }
}
```

### In practice

You can use the `clone_box` method directly or access it through the standard `Clone` trait:

```rust
// using standard clone trait (which the macro handles for you)
let cloned = my_trait_object.clone();

// using the explicit method
let cloned = my_trait_object.clone_box();
```

## The problem

> *For those new to rust's trait object limitations:*

In rust, trait objects (`dyn Trait`) have fundamental limitations due to type erasure and rust's object safety rules. Specifically:

- Trait objects cannot automatically implement marker traits like `Clone`, `Eq`, or
  `Hash`, even when every possible implementor satisfies these bounds, because:
    - The concrete type information is erased at runtime (stored only as a vtable pointer)
    - The compiler cannot verify at compile time that all current and future implementors will satisfy these bounds
    - rust's trait object design intentionally limits which methods are accessible through the vtable

- The `Box<dyn Animal>` cannot be cloned even when all concrete types implement `Clone` because:
    - The `Clone` implementation would need to know the concrete type to call its specific clone method
    - The trait object vtable only contains entries for methods explicitly defined in the trait itself

- Traditional workarounds require:
    - Manual auxiliary traits with explicit `clone_box`-style methods
    - Complex trait bounds and blanket implementations
    - Careful attention to object safety concerns
    - Sometimes unsafe code for downcasting via `Any` or similar mechanisms (with potential performance penalties)

These limitations can make working with trait objects cumbersome in scenarios where operations like cloning (currently handled with the
`clone_box` pattern macro), comparison (todo), or conversion (todo) are needed.

## The Benefits

1. **Type-system friendly**:
   Creates auxiliary trait implementations that work with rust's type system to keep static dispatch for concrete types, only using dynamic dispatch at trait object boundaries where it's unavoidable.

2. **Static type guarantees**:
   Maintains, where possible, rust's type system through trait bounds, for example for the clone_box pattern, by enforcing implementors be
   `Clone + 'static` without runtime checks.
3. **Minimal overhead abstractions**:
   Introduces no overhead beyond the inherent dynamic dispatch required when working with trait objects. Avoids additional indirection layers or heap allocations that would degrade performance compared to a manually written implementation.
4. **Reduces manual boilerplate**:
   Replaces error-prone manual auxiliary traits, blanket implementations, and explicit method forwarding typically needed for the clone_box pattern.

5. **Optimized dispatch implementation**:
   Implements patterns like clone_box using direct trait method calls rather than type erasure techniques such as
   `Any` downcasting. This approach produces more analyzable IR for compiler backends, avoiding additional optimization barriers beyond the inherent limitations of trait objects.

6. **Centralized implementation**:
   Consolidates some potentially complex trait implementation details in a single location, eliminating duplicated logic across different traits requiring the same pattern (and potential for user error/inconsistent implementations because of that).

7. **Focus on preserving object safety**:
   Avoids self-referential methods, associated types without bounds, or other features that would violate object safety requirements.

> **Note:** Performance benchmarks comparing this implementation to manual approaches
> will be added before crates.io release. Current "minimal overhead" claims are based
> on analysis of the generated code rather than quantitative measurements.

## Compatibility

This crate requires rust 1.60.0 or later. We follow a conservative MSRV policy, only bumping the minimum version when significant benefits are gained.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/traitkit/blob/master/LICENSE)
