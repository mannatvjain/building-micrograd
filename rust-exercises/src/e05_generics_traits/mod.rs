// ============================================================
// E05: Generics and Traits (Ch 10.1-10.2)
// ============================================================
use std::fmt;

// --- Exercise 1: Generic function ---
// Implement `largest` for any type that supports comparison.
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    // TODO: Find and return a reference to the largest element.
    todo!()
}

// --- Exercise 2: Generic struct ---
// Define a `Pair<T>` struct with fields `first` and `second`.
// Implement Display for Pair<T> where T: Display.

pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T: fmt::Display> fmt::Display for Pair<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Format as "(first, second)"
        todo!()
    }
}

// --- Exercise 3: Defining a trait ---
// Define a trait `Area` with a method `area(&self) -> f64`.
// Implement it for Circle and RightTriangle.

pub trait Area {
    fn area(&self) -> f64;
}

pub struct Circle {
    pub radius: f64,
}

pub struct RightTriangle {
    pub base: f64,
    pub height: f64,
}

// TODO: Implement Area for Circle and RightTriangle.
// (Stubs so the project compiles — replace todo!() with real logic.)
impl Area for Circle {
    fn area(&self) -> f64 { todo!() }
}
impl Area for RightTriangle {
    fn area(&self) -> f64 { todo!() }
}

// --- Exercise 4: Trait as parameter (impl Trait) ---
// Write a function that accepts anything implementing Area and returns
// a string like "Area: 12.5"
pub fn describe_area(shape: &impl Area) -> String {
    // TODO
    todo!()
}

// --- Exercise 5: Trait bounds with where clause ---
// Write a function that takes two items and returns the one with the
// larger Display string representation.
pub fn longer_display<T>(a: T, b: T) -> T
where
    T: fmt::Display,
{
    // TODO: Compare the Display output lengths. Return the one with
    // the longer string representation.
    todo!()
}

// --- Exercise 6: Returning impl Trait ---
// Return an iterator of even numbers from 0 up to (not including) n.
pub fn evens_up_to(n: i32) -> impl Iterator<Item = i32> {
    // TODO: Use range and filter. Do NOT collect into a Vec.
    // Hint: (0..n).filter(...)
    (0..n).filter(|_| todo!())
}

// --- Exercise 7: Trait with default implementation ---
// Define a trait `Greet` with a method `name(&self) -> &str` (no default)
// and a method `greet(&self) -> String` with a default that uses `name`.

pub trait Greet {
    fn name(&self) -> &str;
    fn greet(&self) -> String {
        format!("Hello, my name is {}!", self.name())
    }
}

pub struct Person {
    pub name: String,
}

impl Greet for Person {
    fn name(&self) -> &str { todo!() }
}

// --- Exercise 8: Conditional method implementation ---
// Implement a method `sum` on Pair<T> only when T: Add + Copy.
use std::ops::Add;

impl<T: Add<Output = T> + Copy> Pair<T> {
    pub fn sum(&self) -> T {
        // TODO
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        assert_eq!(*largest(&[1, 5, 3, 2, 4]), 5);
        assert_eq!(*largest(&["hello", "world", "abc"]), "world");
    }

    #[test]
    fn test_pair_display() {
        let p = Pair { first: 1, second: 2 };
        assert_eq!(format!("{p}"), "(1, 2)");
    }

    #[test]
    fn test_area_circle() {
        let c = Circle { radius: 5.0 };
        assert!((c.area() - 78.53981633974483).abs() < 1e-6);
    }

    #[test]
    fn test_area_triangle() {
        let t = RightTriangle { base: 6.0, height: 4.0 };
        assert_eq!(t.area(), 12.0);
    }

    #[test]
    fn test_describe_area() {
        let c = Circle { radius: 1.0 };
        let desc = describe_area(&c);
        assert!(desc.starts_with("Area: "));
    }

    #[test]
    fn test_longer_display() {
        assert_eq!(longer_display("hi", "hello"), "hello");
        assert_eq!(longer_display(1000, 1), 1000);
    }

    #[test]
    fn test_evens_up_to() {
        let v: Vec<i32> = evens_up_to(10).collect();
        assert_eq!(v, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_greet_default() {
        let p = Person { name: "Alice".to_string() };
        assert_eq!(p.greet(), "Hello, my name is Alice!");
    }

    #[test]
    fn test_pair_sum() {
        let p = Pair { first: 3, second: 7 };
        assert_eq!(p.sum(), 10);
    }
}
