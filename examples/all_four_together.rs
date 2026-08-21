//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

//! All four attributes on one trait, doing what the trait system will not.
//!
//! A plugin registry is the case they were built for: it holds `Box<dyn
//! Plugin>` values it cannot name the types of, and still has to clone the set,
//! compare two of them, ask a plugin for its concrete type, and pass one where
//! a supertrait was expected. Rust gives none of those to a trait object on its
//! own.
//!
//! ```text
//! cargo run --example all_four_together
//! ```

use objkit::{as_super, clone_box, obj_eq};

/// What every plugin is, before anything specific about it.
///
/// `as_super` here rather than on `Plugin`, because it converts *to* the trait
/// it is attached to: on `Described` it writes `as_described`, which is the
/// upcast wanted.
///
/// Not `as_any` as well. Putting that on a supertrait too puts two `as_any`
/// methods in scope on the same value, and every call then has to name which
/// trait it means.
#[as_super]
pub trait Described {
    fn describe(&self) -> String;
}

/// The plugin surface itself.
///
/// Two attributes here, giving four operations the trait system does not:
///
/// - `clone_box` gives `Box<dyn Plugin>` a `Clone`, so the registry can be
///   duplicated.
/// - `obj_eq` gives it a `PartialEq`, so two registries can be compared.
///
/// `as_any` is not written here and is still available, because `obj_eq`
/// applies it itself: it needs the downcast in order to compare. Writing it
/// again is a conflicting implementation, which is what the compiler says if
/// you try, and is worth knowing before you do.
#[clone_box]
#[obj_eq]
pub trait Plugin: Described {
    fn name(&self) -> &'static str;
    fn priority(&self) -> u8;
}

#[derive(Clone, PartialEq)]
struct Formatter {
    width: u8,
}

impl Described for Formatter {
    fn describe(&self) -> String {
        format!("formats to {} columns", self.width)
    }
}

impl Plugin for Formatter {
    fn name(&self) -> &'static str {
        "formatter"
    }

    fn priority(&self) -> u8 {
        10
    }
}

#[derive(Clone, PartialEq)]
struct Linter {
    strict: bool,
}

impl Described for Linter {
    fn describe(&self) -> String {
        if self.strict {
            "refuses everything questionable".to_string()
        } else {
            "warns and moves on".to_string()
        }
    }
}

impl Plugin for Linter {
    fn name(&self) -> &'static str {
        "linter"
    }

    fn priority(&self) -> u8 {
        20
    }
}

fn main() {
    let registry: Vec<Box<dyn Plugin>> = vec![
        Box::new(Formatter {
            width: 100,
        }),
        Box::new(Linter {
            strict: true,
        }),
    ];

    println!("A registry of two plugins, held as trait objects.\n");
    for plugin in &registry {
        println!("{:<12} priority {}", plugin.name(), plugin.priority());
    }

    println!("\n`as_super` on the supertrait: an upcast stable Rust does not give.\n");

    for plugin in &registry {
        // `plugin` is a `&dyn Plugin`. Reaching `Described` through it is an upcast,
        // and without the attribute there is no way to write one.
        let described: &dyn Described = plugin.as_described();
        println!("{:<12} {}", plugin.name(), described.describe());
    }

    println!("\n`as_any`, which `obj_eq` brought along: a downcast to the concrete type.\n");

    for plugin in &registry {
        let concrete = plugin
            .as_any()
            .downcast_ref::<Formatter>()
            .map(|f| format!("a Formatter at {} columns", f.width))
            .or_else(|| {
                plugin
                    .as_any()
                    .downcast_ref::<Linter>()
                    .map(|l| format!("a Linter, strict: {}", l.strict))
            })
            .unwrap_or_else(|| "something else".to_string());

        println!("{:<12} {concrete}", plugin.name());
    }

    println!("\n`clone_box`: the registry duplicated without naming a single type.\n");

    let copy: Vec<Box<dyn Plugin>> = registry.iter().map(|p| p.clone_box()).collect();
    println!(
        "original {} plugins, copy {} plugins",
        registry.len(),
        copy.len()
    );

    println!("\n`obj_eq`: and the two compared, likewise.\n");

    let same =
        registry.len() == copy.len() && registry.iter().zip(copy.iter()).all(|(a, b)| a == b);
    println!("the copy equals the original: {same}");

    // And a registry that genuinely differs does not compare equal, which is what
    // says the comparison is reaching the values rather than the pointers.
    let different: Vec<Box<dyn Plugin>> = vec![
        Box::new(Formatter {
            width: 80,
        }),
        Box::new(Linter {
            strict: true,
        }),
    ];
    let differs = registry.iter().zip(different.iter()).any(|(a, b)| a != b);
    println!("a registry with a different width differs: {differs}");

    // Two plugins of different concrete types are not equal either, which a
    // comparison that only looked at some shared field could get wrong.
    let formatter: Box<dyn Plugin> = Box::new(Formatter {
        width: 100,
    });
    let linter: Box<dyn Plugin> = Box::new(Linter {
        strict: true,
    });
    println!("a formatter equals a linter: {}", formatter == linter);
}
