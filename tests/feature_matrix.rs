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
