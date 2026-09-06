//! What one `==` between two trait objects costs.
//!
//! The arms are the shapes somebody would actually write for this, so the
//! generated code is measured against real alternatives rather than against
//! nothing:
//!
//! - `generated` is `#[obj_eq]`, which is what the crate ships.
//! - `hand_downcast` is the same thing written by hand: a trait with an
//!   `as_any` method and a blanket impl that downcasts the other side, which is
//!   what a consumer would type out without the attribute.
//! - `hash_then_cast` is the shape the attribute expanded to before: hash each
//!   side's `TypeId` to 64 bits, compare the hashes, and cast on the strength
//!   of that. It is here as the competitor the current expansion replaced, so
//!   the replacement is a measured claim rather than an argued one.
//!
//! Three cases per arm, because they take different paths: the same type
//! holding equal values, the same type holding different values, and two
//! different types, which is the case the type check alone answers.

// Every arm needs `criterion`, which is a `std` crate, and the bench has no
// other content. cargo builds a bench target under every feature selection, so
// the gate is per item and `no_std` gets a `main` of its own below.
#[cfg(not(feature = "no_std"))]
use std::hint::black_box;

#[cfg(not(feature = "no_std"))]
use criterion::{criterion_group, criterion_main, Criterion, Throughput};

#[cfg(not(feature = "no_std"))]
mod arms {
    use objkit::obj_eq;

    #[obj_eq]
    pub trait Generated {}

    pub trait Hand {
        fn as_any(&self) -> &dyn core::any::Any;
        fn hand_eq(&self, other: &dyn Hand) -> bool;
    }

    pub trait Hashed {
        fn type_hash(&self) -> u64;
        fn as_object(&self) -> &dyn Hashed;
        fn hashed_eq(&self, other: &dyn Hashed) -> bool;
    }

    #[derive(PartialEq)]
    pub struct A(pub i32);

    #[derive(PartialEq)]
    pub struct B(pub i32);

    macro_rules! implement {
        ($ty:ident) => {
            impl Generated for $ty {}

            impl Hand for $ty {
                #[inline]
                fn as_any(&self) -> &dyn core::any::Any {
                    self
                }

                #[inline]
                fn hand_eq(&self, other: &dyn Hand) -> bool {
                    match other.as_any().downcast_ref::<Self>() {
                        Some(other) => self == other,
                        None => false,
                    }
                }
            }

            impl Hashed for $ty {
                #[inline]
                fn type_hash(&self) -> u64 {
                    use std::hash::{Hash, Hasher};
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    core::any::TypeId::of::<Self>().hash(&mut hasher);
                    hasher.finish()
                }

                #[inline]
                fn as_object(&self) -> &dyn Hashed {
                    self
                }

                #[inline]
                fn hashed_eq(&self, other: &dyn Hashed) -> bool {
                    if self.type_hash() != other.type_hash() {
                        return false;
                    }
                    // SAFETY: this is the arm being measured against, reproduced as it
                    // was, and the two types in this bench hash apart; it is not sound in
                    // general, which is why the crate no longer expands to it.
                    let other =
                        unsafe { &*(other.as_object() as *const dyn Hashed).cast::<Self>() };
                    self == other
                }
            }
        };
    }

    implement!(A);
    implement!(B);
}

#[cfg(not(feature = "no_std"))]
fn one_comparison(c: &mut Criterion) {
    use arms::{Generated, Hand, Hashed, A, B};

    let same_equal = (A(7), A(7));
    let same_unequal = (A(7), A(8));
    let different = (A(7), B(7));

    let mut g = c.benchmark_group("one comparison");
    g.throughput(Throughput::Elements(1));

    for (case, (left, right)) in [
        (
            "same type, equal",
            (
                &same_equal.0 as &dyn Generated,
                &same_equal.1 as &dyn Generated,
            ),
        ),
        ("same type, unequal", (&same_unequal.0, &same_unequal.1)),
        ("different types", (&different.0, &different.1)),
    ] {
        g.bench_function(format!("generated/{case}"), |b| {
            b.iter(|| black_box(left) == black_box(right));
        });
    }

    for (case, (left, right)) in [
        (
            "same type, equal",
            (&same_equal.0 as &dyn Hand, &same_equal.1 as &dyn Hand),
        ),
        ("same type, unequal", (&same_unequal.0, &same_unequal.1)),
        ("different types", (&different.0, &different.1)),
    ] {
        g.bench_function(format!("hand_downcast/{case}"), |b| {
            b.iter(|| black_box(left).hand_eq(black_box(right)));
        });
    }

    for (case, (left, right)) in [
        (
            "same type, equal",
            (&same_equal.0 as &dyn Hashed, &same_equal.1 as &dyn Hashed),
        ),
        ("same type, unequal", (&same_unequal.0, &same_unequal.1)),
        ("different types", (&different.0, &different.1)),
    ] {
        g.bench_function(format!("hash_then_cast/{case}"), |b| {
            b.iter(|| black_box(left).hashed_eq(black_box(right)));
        });
    }

    g.finish();
}

#[cfg(not(feature = "no_std"))]
criterion_group!(benches, one_comparison);
#[cfg(not(feature = "no_std"))]
criterion_main!(benches);

// `criterion_main` generates the entry point and is gated out above, and cargo
// refuses a bench target without a `main` whatever the feature selection.
#[cfg(feature = "no_std")]
fn main() {
    println!("this bench measures through `criterion`, which needs `std`");
}
