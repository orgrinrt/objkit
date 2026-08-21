//! Four crates doing one job, which is the point of them being four.
//!
//! Every other example here is about one thing this crate does. This one is
//! about a shape that needs several: a registry holding plugins as trait
//! objects, where each plugin keeps its own concrete type recoverable, carries
//! an id nothing else can hold, and is filed under a name spelled the same way
//! whatever the author typed.
//!
//! - `objkit` makes the trait object downcastable and clonable, which a bare
//!   `dyn Trait` is neither.
//! - `highroller` supplies the ids. A rolling index rather than a counter, so
//!   an id is cheap and does not have to be handed around to stay unique. It is
//!   a `u16`, which is the point: an id that rolls is meant to be small and
//!   reused, not to be a permanent handle.
//! - `str_extensions` normalises the names. A plugin author writes
//!   `HTTPCacheWarmer`, `http_cache_warmer` or `http-cache-warmer` and the
//!   registry files all three under one key.
//!
//! ```bash
//! cargo run --example a_plugin_registry
//! ```

use objkit::{as_any, clone_box};
use str_extensions::prelude::*;

/// What every plugin can do.
///
/// `as_any` and `clone_box` are two attributes rather than one because they
/// answer different questions: recovering the concrete type, and copying the
/// object without knowing it.
#[as_any]
#[clone_box]
pub trait Plugin {
    /// What the author called it, in whatever case they felt like.
    fn declared_name(&self) -> String;

    /// What it does when the registry runs it.
    fn run(&self) -> String;
}

/// A plugin that warms a cache. One of two concrete types the registry will
/// hold.
#[derive(Clone)]
struct CacheWarmer {
    entries: usize,
}

impl Plugin for CacheWarmer {
    fn declared_name(&self) -> String {
        "HTTPCacheWarmer".to_string()
    }

    fn run(&self) -> String {
        format!("warmed {} entries", self.entries)
    }
}

/// A plugin that collects metrics. The second concrete type, so the registry is
/// genuinely heterogeneous rather than one type behind a trait.
#[derive(Clone)]
struct MetricsCollector {
    interval_seconds: u32,
}

impl Plugin for MetricsCollector {
    fn declared_name(&self) -> String {
        "metrics-collector".to_string()
    }

    fn run(&self) -> String {
        format!("collecting every {}s", self.interval_seconds)
    }
}

/// One registered plugin: the object, the key it was filed under, and its id.
struct Registered {
    key:    String,
    id:     u16,
    plugin: Box<dyn Plugin>,
}

fn main() {
    // Two plugins, spelled differently, registered the same way.
    let sources: Vec<Box<dyn Plugin>> = vec![
        Box::new(CacheWarmer {
            entries: 512,
        }),
        Box::new(MetricsCollector {
            interval_seconds: 30,
        }),
    ];

    let registry: Vec<Registered> = sources
        .into_iter()
        .map(|plugin| {
            Registered {
                // str_extensions: whatever the author typed becomes one spelling. The conversion
                // segments into words first, so it does not care which case it was given, and it
                // returns a `Cow`, borrowing where the input was already in the target case.
                key: plugin.declared_name().to_snake_case().into_owned(),
                // highroller: an id without a counter to pass around.
                id: highroller::rolling_idx(),
                plugin,
            }
        })
        .collect();

    println!("== registered");
    for entry in &registry {
        println!(
            "  {:<22} id {:<4} {}",
            entry.key,
            entry.id,
            entry.plugin.run()
        );
    }

    // The names went in as `HTTPCacheWarmer` and `metrics-collector` and came out
    // in one spelling, which is what makes a key a key.
    println!();
    println!("== the keys are one spelling, whatever was typed");
    for entry in &registry {
        println!(
            "  declared {:<20} -> key {}",
            entry.plugin.declared_name(),
            entry.key
        );
    }

    // objkit's as_any: the concrete type is recoverable from the trait object,
    // which is what lets a registry hold anything and still let a caller ask
    // for one specific thing.
    println!();
    println!("== recovering the concrete type");
    for entry in &registry {
        if let Some(warmer) = entry.plugin.as_any().downcast_ref::<CacheWarmer>() {
            println!(
                "  {} is a CacheWarmer over {} entries",
                entry.key, warmer.entries
            );
        } else if let Some(metrics) = entry.plugin.as_any().downcast_ref::<MetricsCollector>() {
            println!(
                "  {} is a MetricsCollector at {}s",
                entry.key, metrics.interval_seconds
            );
        }
    }

    // The refusal, which is what makes the recovery above mean anything: asking for
    // the wrong type answers None rather than doing something.
    let first = &registry[0];
    println!();
    println!(
        "  asking the first for the wrong type: {:?}",
        first
            .plugin
            .as_any()
            .downcast_ref::<MetricsCollector>()
            .map(|_| "found")
    );

    // objkit's clone_box: copying a trait object without knowing what is behind it.
    // A `Clone` bound would make the trait not object-safe, which is the
    // problem this answers.
    println!();
    println!("== cloning through the trait object");
    let copy = registry[0].plugin.clone_box();
    println!("  original: {}", registry[0].plugin.run());
    println!("  clone:    {}", copy.run());
    println!(
        "  the clone kept its concrete type: {}",
        copy.as_any().downcast_ref::<CacheWarmer>().is_some()
    );

    // highroller's ids are distinct without anything having tracked them.
    println!();
    let ids: Vec<u16> = registry.iter().map(|e| e.id).collect();
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    println!(
        "== {} plugins, {} distinct ids, nothing counted them",
        ids.len(),
        sorted.len()
    );
}
