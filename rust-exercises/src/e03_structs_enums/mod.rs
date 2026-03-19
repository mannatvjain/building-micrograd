// ============================================================
// E03: Structs, Enums, and Pattern Matching (Ch 5-6)
// ============================================================

// --- Exercise 1: Struct basics ---
// Define a struct `Rectangle` with width and height (both f64).
// Implement a method `area` and an associated function `square`.

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    // TODO: Return the area.
    pub fn area(&self) -> f64 {
        todo!()
    }

    // TODO: Create a square (width == height).
    pub fn square(size: f64) -> Rectangle {
        todo!()
    }

    // TODO: Return true if self can hold other (both dimensions larger).
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        todo!()
    }
}

// --- Exercise 2: Enum with data ---
// Define an enum `Shape` with variants:
//   Circle(f64)           — radius
//   Rect(f64, f64)        — width, height
//   Triangle(f64, f64)    — base, height

pub enum Shape {
    Circle(f64),
    Rect(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    // TODO: Compute the area for any shape variant using match.
    pub fn area(&self) -> f64 {
        todo!()
    }
}

// --- Exercise 3: Option<T> ---
// Return the element at `index` if it exists, None otherwise.
// Do NOT use .get() — write it manually.
pub fn safe_index(v: &[i32], index: usize) -> Option<i32> {
    // TODO
    todo!()
}

// --- Exercise 4: match exhaustiveness ---
// Write a function that converts a u8 grade to a letter.
// 90-100 => "A", 80-89 => "B", 70-79 => "C", 60-69 => "D", _ => "F"
pub fn grade_letter(score: u8) -> &'static str {
    // TODO: Use match with range patterns.
    todo!()
}

// --- Exercise 5: if let ---
// Given an Option<String>, print the value if Some, do nothing if None.
// Return true if it was Some, false if None.
pub fn check_optional(opt: Option<String>) -> bool {
    // TODO: Use if let.
    todo!()
}

// --- Exercise 6: Nested enums ---
// Model a simple expression tree.
#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
}

impl Expr {
    // TODO: Recursively evaluate the expression.
    pub fn eval(&self) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle { width: 3.0, height: 4.0 };
        assert_eq!(r.area(), 12.0);
    }

    #[test]
    fn test_rectangle_square() {
        let s = Rectangle::square(5.0);
        assert_eq!(s.area(), 25.0);
    }

    #[test]
    fn test_can_hold() {
        let r1 = Rectangle { width: 10.0, height: 8.0 };
        let r2 = Rectangle { width: 5.0, height: 3.0 };
        assert!(r1.can_hold(&r2));
        assert!(!r2.can_hold(&r1));
    }

    #[test]
    fn test_shape_area() {
        let c = Shape::Circle(5.0);
        assert!((c.area() - 78.53981633974483).abs() < 1e-6);

        let r = Shape::Rect(3.0, 4.0);
        assert_eq!(r.area(), 12.0);

        let t = Shape::Triangle(6.0, 3.0);
        assert_eq!(t.area(), 9.0);
    }

    #[test]
    fn test_safe_index() {
        let v = vec![10, 20, 30];
        assert_eq!(safe_index(&v, 1), Some(20));
        assert_eq!(safe_index(&v, 5), None);
    }

    #[test]
    fn test_grade_letter() {
        assert_eq!(grade_letter(95), "A");
        assert_eq!(grade_letter(85), "B");
        assert_eq!(grade_letter(75), "C");
        assert_eq!(grade_letter(65), "D");
        assert_eq!(grade_letter(50), "F");
    }

    #[test]
    fn test_check_optional() {
        assert!(check_optional(Some("hi".to_string())));
        assert!(!check_optional(None));
    }

    #[test]
    fn test_expr_eval() {
        // (2 + 3) * -(4)
        let expr = Expr::Mul(
            Box::new(Expr::Add(
                Box::new(Expr::Num(2.0)),
                Box::new(Expr::Num(3.0)),
            )),
            Box::new(Expr::Neg(Box::new(Expr::Num(4.0)))),
        );
        assert_eq!(expr.eval(), -20.0);
    }
}
