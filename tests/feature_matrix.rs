//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------

//! Each feature configuration either builds, or refuses in words a consumer can act on.
//!
//! `no_std` did not build at all before this test existed. `objkit-macros` carried
//! `#![cfg_attr(feature = "no_std", no_std)]`, which a procedural macro crate cannot honour,
//! since it runs on the host inside the compiler. Nothing caught it because every test ran
//! under the default features.

use std::process::Command;

/// Runs `cargo check` for one configuration and gives back its stderr.
fn check(args: &[&str]) -> (bool, String) {
    let output = Command::new(env!("CARGO"))
        .arg("check")
        .arg("--quiet")
        .args(args)
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        // A separate target dir, so these do not fight the outer `cargo test` for the lock
        // and do not invalidate its artifacts by rebuilding with other features.
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/feature-matrix"))
        .output()
        .expect("cargo runs");
    (
        output.status.success(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

#[test]
fn the_default_configuration_builds() {
    let (ok, err) = check(&[]);
    assert!(ok, "default features build:\n{err}");
}

#[test]
fn no_std_builds() {
    let (ok, err) = check(&["--no-default-features", "--features", "no_std"]);
    assert!(ok, "no_std builds:\n{err}");
}

#[test]
fn neither_feature_is_refused_in_the_consumer_s_own_vocabulary() {
    let (ok, err) = check(&["--no-default-features"]);
    assert!(!ok, "selecting neither feature cannot build");
    assert!(
        err.contains("default-features = false") && err.contains("no_std"),
        "the refusal says what to do about it, rather than naming an internal feature the \
         consumer has never heard of:\n{err}"
    );
}

#[test]
fn both_features_at_once_are_refused_with_the_reason() {
    // `--features no_std` without `--no-default-features` leaves `std` on as well, which is
    // the mistake this message exists for.
    let (ok, err) = check(&["--features", "no_std"]);
    assert!(!ok, "both features at once cannot build");
    assert!(
        err.contains("exclusive"),
        "the refusal says why:\n{err}"
    );
}

/// Builds a throwaway crate whose body is `body`, against this crate at `features`.
///
/// The allocation axis cannot be seen from inside this crate: the macros expand in somebody
/// else's, and `no_alloc` is a claim about what those expansions may name. Every case below
/// therefore compiles a consumer rather than checking this crate.
fn consumer_compiles(name: &str, features: &str, attrs: &str, body: &str) -> (bool, String) {
    use std::fs;
    use std::path::PathBuf;

    let root = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers")).join(name);
    fs::create_dir_all(root.join("src")).expect("the consumer directory");

    let features_list =
        features.split(',').map(|f| format!("\"{f}\"")).collect::<Vec<_>>().join(", ");

    fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n\
             [dependencies.objkit]\npath = \"{crate_dir}\"\ndefault-features = false\n\
             features = [{features_list}]\n\n[workspace]\n",
            crate_dir = env!("CARGO_MANIFEST_DIR"),
        ),
    )
    .expect("the consumer manifest");

    fs::write(root.join("src").join("lib.rs"), format!("{attrs}\n{body}\n"))
        .expect("the consumer source");

    let output = Command::new(env!("CARGO"))
        .args(["check", "--quiet"])
        .current_dir(&root)
        .env("CARGO_TARGET_DIR", concat!(env!("CARGO_MANIFEST_DIR"), "/target/consumers/target"))
        .output()
        .expect("cargo runs");

    (output.status.success(), String::from_utf8_lossy(&output.stderr).to_string())
}

/// A trait with `obj_eq`, and two implementors to compare.
const COMPARABLE: &str = r#"
#[objkit::obj_eq]
pub trait Value {
    fn value(&self) -> i32;
}

#[derive(PartialEq)]
pub struct One;
impl Value for One { fn value(&self) -> i32 { 1 } }
"#;

#[test]
fn the_allocation_axis_builds_under_every_selection() {
    for features in ["no_alloc", "no_alloc,no_std"] {
        let (ok, err) = check(&["--no-default-features", "--features", features]);
        assert!(ok, "{features} builds:\n{err}");
    }
}

#[test]
fn obj_eq_compares_references_under_every_selection() {
    // The operation is comparing two `&dyn Trait`, and it survives `no_alloc` because it
    // never needed an allocator. Only the convenience impls over `Box<dyn Trait>` did.
    let body =
        format!("{COMPARABLE}\npub fn same(a: &dyn Value, b: &dyn Value) -> bool {{ a == b }}\n");

    for features in ["std", "no_std", "no_alloc"] {
        let attrs = if features == "std" { "" } else { "#![no_std]\nextern crate alloc;" };
        let (ok, err) = consumer_compiles(&format!("obj_eq_{features}"), features, attrs, &body);
        assert!(ok, "`obj_eq` compares references at {features}:\n{err}");
    }
}

#[test]
fn obj_eq_compares_boxes_only_where_there_is_an_allocator() {
    let body = format!(
        "{COMPARABLE}\npub fn same(a: alloc::boxed::Box<dyn Value>, \
         b: alloc::boxed::Box<dyn Value>) -> bool {{ a == b }}\n"
    );
    let attrs = "#![no_std]\nextern crate alloc;";

    let (ok, err) = consumer_compiles("obj_eq_boxed_no_std", "no_std", attrs, &body);
    assert!(ok, "`PartialEq for Box<dyn Value>` exists under `no_std`:\n{err}");

    // The half a build check cannot reach: an impl emitted anyway passes every positive
    // test above, and a feature that gates nothing is invisible to all of them.
    let (ok, _) = consumer_compiles("obj_eq_boxed_no_alloc", "no_alloc", attrs, &body);
    assert!(!ok, "`PartialEq for Box<dyn Value>` must be absent under `no_alloc`");
}

#[test]
fn clone_box_is_absent_under_no_alloc() {
    let body = "#[objkit::clone_box]\npub trait Shape {\n    fn sides(&self) -> u8;\n}\n";

    let (ok, err) =
        consumer_compiles("clone_box_present", "no_std", "#![no_std]\nextern crate alloc;", body);
    assert!(ok, "`clone_box` is present under `no_std`:\n{err}");

    // A boxed clone is exactly what an allocator is for, so there is nothing to offer
    // without one. Absent rather than present-and-failing, so a consumer finds out at the
    // import rather than inside an expansion.
    let (ok, err) = consumer_compiles("clone_box_absent", "no_alloc", "#![no_std]", body);
    assert!(!ok, "`clone_box` must be absent under `no_alloc`");
    assert!(err.contains("clone_box"), "the error names it:\n{err}");
}

#[test]
fn as_any_and_as_super_are_unaffected_by_the_allocation_axis() {
    // The control for the two tests above. Without it they would pass just as well against
    // a `no_alloc` that had removed everything, and neither of these ever needed an
    // allocator, so neither may change across the three positions.
    let body = r#"
#[objkit::as_any]
pub trait Named {
    fn name(&self) -> &'static str;
}

#[objkit::as_super]
pub trait Base {
    fn base(&self) -> u8;
}

pub struct Thing;
impl Named for Thing { fn name(&self) -> &'static str { "thing" } }
impl Base for Thing { fn base(&self) -> u8 { 1 } }

pub fn downcast(value: &dyn Named) -> bool {
    value.as_any().downcast_ref::<Thing>().is_some()
}
"#;

    for features in ["std", "no_std", "no_alloc"] {
        let attrs = if features == "std" { "" } else { "#![no_std]\nextern crate alloc;" };
        let (ok, err) = consumer_compiles(&format!("untouched_{features}"), features, attrs, body);
        assert!(ok, "`as_any` and `as_super` work at {features}:\n{err}");
    }
}
