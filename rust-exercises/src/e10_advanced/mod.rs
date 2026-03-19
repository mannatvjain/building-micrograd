// ============================================================
// E10: Advanced Features (Ch 19-20)
// ============================================================

// --- Exercise 1: Trait objects (dyn) ---
// Create a function that takes a Vec of different shapes (as trait objects)
// and returns the total area.

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Square {
    pub side: f64,
}
pub struct Disk {
    pub radius: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 { todo!() }
}
impl Shape for Disk {
    fn area(&self) -> f64 { todo!() }
}

pub fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    // TODO: Sum the areas.
    todo!()
}

// --- Exercise 2: Associated types ---
// Define a trait `Container` with an associated type `Item`.
// Implement it for a simple wrapper.

pub trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
    fn last(&self) -> Option<&Self::Item>;
    fn len(&self) -> usize;
}

pub struct VecContainer<T> {
    pub items: Vec<T>,
}

impl<T> Container for VecContainer<T> {
    type Item = T;
    fn first(&self) -> Option<&T> { todo!() }
    fn last(&self) -> Option<&T> { todo!() }
    fn len(&self) -> usize { todo!() }
}

// --- Exercise 3: Operator overloading ---
// Implement Add for a Point struct.
use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        // TODO
        todo!()
    }
}

// --- Exercise 4: Newtype pattern ---
// Implement Display for a wrapper around Vec<String> that joins with commas.
use std::fmt;

pub struct CommaSeparated(pub Vec<String>);

impl fmt::Display for CommaSeparated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: Display as "a, b, c"
        todo!()
    }
}

// --- Exercise 5: Type aliases ---
// Create a type alias for a common Result type.
pub type AppResult<T> = Result<T, String>;

pub fn parse_and_double(s: &str) -> AppResult<i32> {
    // TODO: Parse s as i32, double it, use the type alias.
    todo!()
}

// --- Exercise 6: Function pointers ---
// Write a function that takes a function pointer and applies it to each
// element of a vec.
pub fn apply_fn(v: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    // TODO
    todo!()
}

// --- Exercise 7: Returning closures ---
// Return a boxed closure that multiplies by n.
pub fn multiplier(n: i32) -> Box<dyn Fn(i32) -> i32> {
    // TODO
    todo!()
}

// --- Exercise 8: Simple macro ---
// Write a macro `avec!` that works like vec! but only for 2 elements,
// returning them in reversed order.
// Usage: avec![1, 2] => vec![2, 1]
macro_rules! avec {
    // TODO: Define the macro pattern.
    ($a:expr, $b:expr) => {
        todo!()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_area() {
        let shapes: Vec<Box<dyn Shape>> = vec![
            Box::new(Square { side: 3.0 }),
            Box::new(Disk { radius: 1.0 }),
        ];
        let total = total_area(&shapes);
        assert!((total - (9.0 + std::f64::consts::PI)).abs() < 1e-6);
    }

    #[test]
    fn test_container() {
        let c = VecContainer { items: vec![1, 2, 3] };
        assert_eq!(c.first(), Some(&1));
        assert_eq!(c.last(), Some(&3));
        assert_eq!(c.len(), 3);
    }

    #[test]
    fn test_point_add() {
        let p = Point { x: 1.0, y: 2.0 } + Point { x: 3.0, y: 4.0 };
        assert_eq!(p, Point { x: 4.0, y: 6.0 });
    }

    #[test]
    fn test_comma_separated() {
        let cs = CommaSeparated(vec!["a".into(), "b".into(), "c".into()]);
        assert_eq!(format!("{cs}"), "a, b, c");
    }

    #[test]
    fn test_parse_and_double() {
        assert_eq!(parse_and_double("21"), Ok(42));
        assert!(parse_and_double("abc").is_err());
    }

    #[test]
    fn test_apply_fn() {
        assert_eq!(apply_fn(&[1, 2, 3], |x| x * x), vec![1, 4, 9]);
    }

    #[test]
    fn test_multiplier() {
        let times3 = multiplier(3);
        assert_eq!(times3(5), 15);
        assert_eq!(times3(10), 30);
    }

    #[test]
    fn test_avec_macro() {
        let v: Vec<i32> = avec![1, 2];
        assert_eq!(v, vec![2, 1]);
    }
}
