# `objkit`

> Attribute macros for the trait-object operations rust doesn't give you directly: cloning, comparison and conversion.

A `Box<dyn Trait>` can't be cloned or compared and can't be turned back into a concrete type, even when every implementor could do all three. `objkit` generates the auxiliary code for that from a single attribute on the trait, `clone_box` for cloning, `obj_eq` for equality, `as_any` for downcasting and `as_super` for handing out a supertrait reference.

## Usage

Not published to crates.io yet, so it is a git dependency for now.

```toml
[dependencies]
objkit = { git = "https://github.com/orgrinrt/objkit", branch = "dev" }
```

```rust
use objkit::clone_box;

#[clone_box]
pub trait Animal {
    fn speak(&self) -> String;
}

#[derive(Clone)]
struct Dog;

impl Animal for Dog {
    fn speak(&self) -> String { "Woof".to_string() }
}

let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog)];
let copy = animals.clone();
```

The macro makes `Box<dyn Animal>` implement `Clone`, and `clone_box()` is there as an explicit method as well. Implementors have to be `Clone + 'static`. Only `clone_box` is stable so far, the other three work and are tested but their surface isn't settled.

The `examples/` directory has one for each attribute and one with all four on a trait, and `cargo test` runs every one and checks what it printed. `a_plugin_registry.rs` combines `objkit` with `highroller` and `str_extensions` to build a registry of mixed types whose entries stay recoverable.

## Features

`std` is the default and uses `Box` from std. `no_std` switches the generated code over to `alloc`, which also means turning the defaults off, and `no_alloc` implies `no_std` and drops what needs an allocator, so `clone_box` is gone and `obj_eq` only compares `&dyn Trait`. `tests/feature_matrix.rs` builds a consumer crate under each, since the macros expand somewhere else and that is the only place it shows. The minimum supported rust is 1.71, minor versions may break and patch versions don't.

## Costs

It is a procedural macro dependency, so it adds some compile time, and the generated helper traits live in your crate. There are no benchmarks against hand-written versions yet.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/objkit/blob/dev/LICENSE)
