//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

//! The examples are built by `cargo test` and never run by it, so they are run here.
//!
//! For this crate the risk is that an operation appears to work and is comparing the wrong
//! thing. Two boxed trait objects compare equal under a `PartialEq` that only looked at the
//! vtable pointer, and a downcast that always returned `None` reads as a type mismatch
//! rather than as a broken macro. So the checks are on what the values say, not on whether
//! the example ran.

use std::process::Command;

/// Runs one example and returns what it printed.
fn run_example(name: &str) -> String {
    let output = Command::new(env!("CARGO"))
        .args(["run", "-q", "--example", name])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/examples"))
        .output()
        .unwrap_or_else(|e| panic!("could not run example {name}: {e}"));

    assert!(
        output.status.success(),
        "example {name} exited {}\n--- stderr\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );

    String::from_utf8(output.stdout).expect("example printed something that is not utf-8")
}

#[test]
fn every_single_attribute_example_runs() {
    // One per attribute, and each has to have produced its own output rather than merely
    // exited zero.
    for (name, expected) in [
        ("as_any", "Woof"),
        ("as_super", "Woof"),
        ("clone_box", "Woof"),
        ("obj_eq", "true"),
    ] {
        let out = run_example(name);
        assert!(
            out.contains(expected),
            "example {name} did not print {expected:?}:\n{out}",
        );
    }
}

#[test]
fn all_four_together_shows_each_operation_working() {
    let out = run_example("all_four_together");

    // The upcast, reaching a supertrait method through a `&dyn Plugin`.
    assert!(out.contains("formatter    formats to 100 columns"), "no upcast in:\n{out}");

    // The downcast, which recovered the concrete type and read a field off it. A downcast
    // that always returned `None` would print "something else" and still look plausible.
    assert!(out.contains("a Formatter at 100 columns"), "no downcast in:\n{out}");
    assert!(out.contains("a Linter, strict: true"), "no downcast in:\n{out}");
    assert!(!out.contains("something else"), "a downcast failed in:\n{out}");

    // The clone, and the comparison, with both directions. The negatives are what say the
    // comparison reaches the values: a `PartialEq` looking only at the vtable pointer would
    // report the copy equal and the differing registry equal too.
    assert!(out.contains("the copy equals the original: true"), "no clone in:\n{out}");
    assert!(
        out.contains("a registry with a different width differs: true"),
        "the comparison did not see a differing value:\n{out}",
    );
    assert!(
        out.contains("a formatter equals a linter: false"),
        "two different concrete types compared equal:\n{out}",
    );
}
