# objkit

<div align="center" style="text-align: center;">

[![GitHub Stars](https://img.shields.io/github/stars/orgrinrt/objkit.svg)](https://github.com/orgrinrt/objkit/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/orgrinrt/objkit.svg)](https://github.com/orgrinrt/objkit/issues)
[![Latest Version](https://img.shields.io/badge/version-0.0.2-red.svg?label=latest)](https://github.com/orgrinrt/objkit)
![GitHub last commit](https://img.shields.io/github/last-commit/orgrinrt/objkit?color=%23009689&link=https%3A%2F%2Fgithub.com%2Forgrinrt%2Fobjkit)

> A toolkit providing convenient abstractions for trait object operations that aren't supported by rust's trait system directly, such as cloning, comparison, and conversion

</div>

## Examples

`examples/` runs. `cargo test` executes every one and asserts on what it printed.

| Example | What it shows |
|---|---|
| `as_any.rs` | recovering a concrete type from a trait object, and the downcast that refuses |
| `as_super.rs` | two implementors held behind one supertrait reference |
| `clone_box.rs` | copying a trait object without knowing what is behind it |
| `obj_eq.rs` | comparing trait objects |
| `all_four_together.rs` | the four attributes on one trait |
| `a_plugin_registry.rs` | **four crates doing one job**: `objkit` for the trait objects, `highroller` for ids, `str_extensions` to file `HTTPCacheWarmer` and `metrics-collector` under one key |

The last one is the only example here that is about something other than this crate. It exists
because the shape it shows, a heterogeneous registry whose entries stay recoverable, needs more
than one of these crates and is not visible from any of them alone.

## Features

| Feature     | Status      | Description                                 |
|-------------|-------------|---------------------------------------------|
| `clone_box` | ✅ Stable    | `clone_box` pattern                         |
| `obj_eq`    | 🚧 Unstable | equality comparisons for trait objects      |
| `as_super`  | 🚧 Unstable | a shorthand for `as_foo(&self) -> &dyn Foo` |
| `as_any`    | 🚧 Unstable | downcasting to `Any` for trait objects      |

## Usage

This crate provides procedural macros that enhance rust traits by enabling operations that aren't natively supported for trait objects. Currently, the sole stable feature is the
`clone_box` attribute, which enables cloning of trait objects with minimal abstraction overhead beyond the unavoidable dynamic dispatch. The `obj_eq`, `as_super` and
`as_any` attributes work too, each with integration and edge-case tests, though their surface is
not settled yet.

```rust
use objkit::clone_box;

#[clone_box]
pub trait MyTrait {}
```

## Example

Here's a simple example showing how to use the `clone_box` attribute to create clonable trait objects without the boilerplate:

```rust
use objkit::clone_box;

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
# use objkit::clone_box;
# #[clone_box]
# trait Animal {
#     fn speak(&self) -> String;
# }
# #[derive(Clone)]
# struct Dog;
# impl Animal for Dog {
#     fn speak(&self) -> String { "Woof".to_string() }
# }
# let my_trait_object: Box<dyn Animal> = Box::new(Dog);
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

These limitations can make working with trait objects cumbersome in scenarios where operations like cloning (handled with the
`clone_box` pattern macro), comparison (the `obj_eq` macro), or conversion (the `as_super` and `as_any` macros) are needed.

## Pros & Cons

<details>
<summary>Click to expand initial listing (not necessarily accurate at this point anymore)</summary>

### Pros

1. **Type-system friendly**:
   Creates auxiliary trait implementations that work with rust's type system to keep static dispatch for concrete types, only using dynamic dispatch at trait object boundaries where it's unavoidable.

2. **Static type guarantees**:
   Maintains, where possible, rust's type system through trait bounds, for example for the `clone_box` pattern, by enforcing implementors be
   `Clone + 'static` without runtime checks.
3. **Minimal overhead abstractions**:
   ~~Introduces no overhead beyond the inherent dynamic dispatch required when working with trait objects. Avoids additional indirection
   layers or heap allocations that would degrade performance compared to a manually written implementation.~~ **NOTE: right now this is a
   work in progress and does not necessarily hold true**
4. **Reduces manual boilerplate**:
   Replaces error-prone manual auxiliary traits, blanket implementations, and explicit method forwarding typically needed for the `clone_box` pattern.

5. **Optimized dispatch implementation**:
   Implements patterns like `clone_box` using direct trait method calls rather than type erasure techniques such as
   `Any` downcasting. This approach produces more analyzable IR for compiler backends, avoiding additional optimization barriers beyond the inherent limitations of trait objects.

6. **Centralized implementation**:
   Consolidates some potentially complex trait implementation details in a single location, eliminating duplicated logic across different traits requiring the same pattern (and potential for user error/inconsistent implementations because of that).

7. **Focus on preserving object safety**:
   Avoids self-referential methods, associated types without bounds, or other features that would violate object safety requirements.

### Cons

1. **Procedural Macro Dependency**:
   Adds a procedural macro dependency to your project, which will increase compile times, even if only slightly. They can easily build up, so be mindful of that.

2. **Additional Generated Traits**:
   Creates auxiliary traits in your codebase that could potentially lead to name conflicts or increase the binary size.

3. **Implicit Code Generation**:
   The auto-generated implementations may make it less obvious what's happening under the hood compared to manual implementations. But that's also a pro. It's a two-edged sword.

4. **Still Developing Features**:
   Currently only implements the `clone_box` pattern, with other patterns still in planning.

5. **Trait Object Limitations**:
   Still bound by rust's fundamental trait object constraints. Not a magic bullet, just a convenience for some common patterns.

6. **Strict Trait Bounds**:
   Imposes specific trait bounds (like
   `Clone + 'static`) which might be more restrictive than a bespoke manual implementation for some cases.

7. **Learning Curve for Debugging**:
   Requires understanding the underlying pattern to effectively work through possible issues. Some of the quirks that come with the territory may not be immediately obvious to those who don't know the pattern, which can cause frustration.

</details>

> **Note:** Performance benchmarks comparing this implementation to manual approaches
> will be added before the first minor release `0.1`. Current "minimal overhead" claims are based
> on analysis of the generated code rather than quantitative measurements.

## Compatibility

This crate requires rust `1.64.0` or later.

For practical reasons, we pin the msrv there to use cargo's stabilized
`workspace-inheritance` feature, but also to remain fairly compatible.

### Feature flags

The crate has two cargo features: `std` (enabled by default) uses `std::boxed::Box`, while
`no_std` is intended to switch to `alloc::boxed::Box` for embedded or similarly constrained
targets. Exactly one of the two is meant to be on, and because `std` is a default feature,
selecting `no_std` also means turning the defaults off.

**`no_std` does not build today.** `cargo build --no-default-features --features no_std` fails
while compiling `objkit-macros`, which applies `#![cfg_attr(feature = "no_std", no_std)]` to
itself and then still calls `format!` and `to_string`. A procedural macro crate runs on the host
at compile time, so it has no reason to be `no_std` in the first place; the flag it needs to
propagate is the one choosing the `Box` path in the code it *generates*. Until that is separated,
`std` (the default) is the only configuration that compiles, and `--no-default-features` alone and
`--all-features` both fail as well.

The `as_any` attribute is the one piece that is ready for the `no_std` case: it names
`core::any::Any`, which is the same type `std` re-exports, and its expansion allocates nothing.
The `obj_eq` attribute is not: its `no_std` arm expands to `siphasher` and `typeable` paths, and
those are dependencies of the macro crate rather than of `objkit`, so they would not be in scope
at the call site.

### Versioning policy

Minor versions may have breaking changes, which can include bumping msrv.

Patch versions are backwards compatible, so using version specifiers such as `~x.y` or `^x.y.0` is safe.

## Support

Whether you use this project, have learned something from it, or just like it, please consider supporting it by buying me a coffee, so I can dedicate more time on open-source projects like this :)

<a href="https://buymeacoffee.com/orgrinrt" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: auto !important;width: auto !important;" ></a>

## Allocation

Three positions, and each is a feature. `tests/feature_matrix.rs` compiles a real consumer
crate under every one, which is the only place any of this can be seen: the macros expand
somewhere else.

| Feature | What is available |
|---|---|
| `std` | Everything. |
| `no_std` | Everything, against `alloc`. |
| `no_alloc` | What needs no allocator at all. |

`no_alloc` implies `no_std` and is a real subset rather than a rename:

- `as_any` and `as_super` are untouched, because neither ever needed an allocator.
- `obj_eq` still compares `&dyn Trait`, which is the operation. It loses the two impls over
  `Box<dyn Trait>`, which were a convenience over that.
- `clone_box` is absent. A boxed clone is exactly what an allocator is for, so there is
  nothing to offer without one, and it is gone at the import rather than failing inside an
  expansion.

The `no_std` path for `obj_eq` had never worked. It reached for `typeable::TypeId`, which is
a private re-import of `std::any::TypeId` inside a crate that is itself `std`, so the path
did not resolve and the crate it came from defeated the purpose twice over. `core::any::TypeId`
has been in core since 1.0 and is what it uses now, which drops that dependency entirely.
Nothing had caught it because no test compiled a `#![no_std]` consumer that used `obj_eq`.

## License

> The project is licensed under the **Mozilla Public License 2.0**.

`SPDX-License-Identifier: MPL-2.0`

> You can check out the full license [here](https://github.com/orgrinrt/objkit/blob/main/LICENSE)
