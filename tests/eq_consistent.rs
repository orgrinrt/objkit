//------------------------------------------------------------------------------
// Copyright (c) 2025 orgrinrt (orgrinrt@ikiuni.dev)
//                    Hiisi Digital Oy (contact@hiisi.digital)
// SPDX-License-Identifier: MPL-2.0
//------------------------------------------------------------------------------
#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;

// NOTE: this is especially to test the no_std feature is also consistent

use objkit::obj_eq;

// Simple scalar value trait
#[obj_eq]
pub trait Value {
    fn get_value(&self) -> i32;
}

#[derive(PartialEq)]
struct IntValue(i32);

impl Value for IntValue {
    fn get_value(&self) -> i32 {
        self.0
    }
}

// Trait with multiple methods
#[obj_eq]
pub trait Shape {
    fn area(&self) -> f32;
    fn name(&self) -> &str;
}

#[derive(PartialEq)]
struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        ::std::f32::consts::PI * self.radius * self.radius
    }
    fn name(&self) -> &str {
        "Circle"
    }
}

#[derive(PartialEq)]
struct Square {
    side: f32,
}

impl Shape for Square {
    fn area(&self) -> f32 {
        self.side * self.side
    }
    fn name(&self) -> &str {
        "Square"
    }
}

#[obj_eq]
pub trait Container<T> {
    fn contains(&self, value: &T) -> bool;
}

#[derive(PartialEq)]
struct RangeContainer {
    min: i32,
    max: i32,
}

impl Container<i32> for RangeContainer {
    fn contains(&self, value: &i32) -> bool {
        *value >= self.min && *value <= self.max
    }
}

// Empty marker trait
#[obj_eq]
pub trait Marker {}

#[derive(PartialEq)]
struct MarkerA;

#[derive(PartialEq)]
struct MarkerB;

impl Marker for MarkerA {}
impl Marker for MarkerB {}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_equality() {
        let v1 = IntValue(42);
        let v2 = IntValue(42);
        let v3 = IntValue(99);

        let v1_ref = &v1 as &dyn Value;
        let v2_ref = &v2 as &dyn Value;
        let v3_ref = &v3 as &dyn Value;

        assert!(v1_ref == v2_ref, "Same values should be equal");
        assert!(v1_ref != v3_ref, "Different values should not be equal");
        assert!(v2_ref != v3_ref, "Different values should not be equal");

        // Test method still works
        assert_eq!(v1_ref.get_value(), 42);
    }

    #[test]
    fn test_shape_equality() {
        let circle1 = Circle {
            radius: 5.0,
        };
        let circle2 = Circle {
            radius: 5.0,
        };
        let circle3 = Circle {
            radius: 10.0,
        };
        let square = Square {
            side: 7.0,
        };

        let c1_ref = &circle1 as &dyn Shape;
        let c2_ref = &circle2 as &dyn Shape;
        let c3_ref = &circle3 as &dyn Shape;
        let s_ref = &square as &dyn Shape;

        assert!(c1_ref == c2_ref, "Same circles should be equal");
        assert!(c1_ref != c3_ref, "Different circles should not be equal");
        assert!(c1_ref != s_ref, "Circle and square should not be equal");

        // Test methods still work
        // Stated as the relation rather than as a decimal, so it does not have to be
        // recomputed by hand when the constant behind `area` changes. It did: this read
        // 78.53975, which is 3.14159 times the radius squared rather than pi times it.
        let expected = ::std::f32::consts::PI * 5.0 * 5.0;
        assert!((c1_ref.area() - expected).abs() < 0.00001);
        assert_eq!(c1_ref.name(), "Circle");
        assert_eq!(s_ref.name(), "Square");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_boxed_objects() {
        let v1 = Box::new(IntValue(42)) as Box<dyn Value>;
        let v2 = Box::new(IntValue(42)) as Box<dyn Value>;
        let v3 = Box::new(IntValue(99)) as Box<dyn Value>;

        assert!(v1 == v2, "Same boxed values should be equal");
        assert!(v1 != v3, "Different boxed values should not be equal");

        // Test method still works
        assert_eq!(v1.get_value(), 42);
    }

    #[test]
    fn test_marker_traits() {
        let a1 = MarkerA;
        let a2 = MarkerA;
        let b = MarkerB;

        let a1_ref = &a1 as &dyn Marker;
        let a2_ref = &a2 as &dyn Marker;
        let b_ref = &b as &dyn Marker;

        assert!(a1_ref == a2_ref, "Same marker objects should be equal");
        assert!(
            a1_ref != b_ref,
            "Different marker objects should not be equal"
        );
    }

    #[test]
    fn test_generic_trait() {
        let range1 = RangeContainer {
            min: 0,
            max: 100,
        };
        let range2 = RangeContainer {
            min: 0,
            max: 100,
        };
        let range3 = RangeContainer {
            min: 50,
            max: 150,
        };

        let r1_ref = &range1 as &dyn Container<i32>;
        let r2_ref = &range2 as &dyn Container<i32>;
        let r3_ref = &range3 as &dyn Container<i32>;

        assert!(r1_ref == r2_ref, "Same container ranges should be equal");
        assert!(
            r1_ref != r3_ref,
            "Different container ranges should not be equal"
        );

        // Test method still works
        assert!(r1_ref.contains(&50));
        assert!(!r1_ref.contains(&150));
    }
}
