// ============================================================
// E01: Variables, Types, Functions, and Control Flow (Ch 3)
// ============================================================

// --- Exercise 1: Mutability ---
// Fix this function so it compiles.
pub fn mutability() -> i32 {
    // TODO: This won't compile. Make `x` mutable.
    let mut x = 5;
    x = x + 1;
    x
}

// --- Exercise 2: Shadowing ---
// Use shadowing to transform `x` through multiple types.
pub fn shadowing() -> &'static str {
    let x = 5;
    let x = x*2;
    let x = if x == 10 { "twenty" } else { "not ten" };
    return x;
    // TODO: Shadow x to be x * 2 (still i32), then shadow x to be
    // the string "twenty" if x == 10, or "not ten" otherwise.
    // Return the final x.
    todo!()
}

// --- Exercise 3: Tuple destructuring ---
// Return the second and fourth elements of a 5-tuple.
pub fn tuple_extract(t: (i32, f64, i32, bool, char)) -> (f64, bool) {
    // TODO: Destructure the tuple and return (second, fourth).
    return (t.1, t.3);
    todo!()
}

// --- Exercise 4: Array and slice ---
// Return the sum of all elements in the slice.
pub fn sum_slice(s: &[i32]) -> i32 {
    // TODO: Iterate over the slice and sum elements.
    let mut sum = 0;
    for i in s {
        sum += i;
    }
    return sum;
}

// --- Exercise 5: Functions and expressions ---
// Rust is expression-based. Write a function that returns the larger
// of two i32 values using an if *expression* (no `return` keyword).
pub fn max_of(a: i32, b: i32) -> i32 {
    if a > b {
        a
    }
    else {
        b
    }
}

// --- Exercise 6: Loops ---
// Use a `loop` with `break` to return the first power of 2 >= n.
pub fn next_power_of_two(n: u32) -> u32 {
    let mut result = 1;
    loop { 
        if result >= n {
            break result;
        }
        result *= 2;
    }
}

// --- Exercise 7: Pattern matching in control flow ---
// FizzBuzz using match. Return "Fizz", "Buzz", "FizzBuzz", or the
// number as a String.
pub fn fizzbuzz(n: u32) -> String {
    // TODO: Use match on (n % 3, n % 5) to determine the result.
    match (n % 3, n % 5) { 
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"), 
        (_, 0) => String::from("Buzz"),
        _ => n.to_string(),
    }   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutability() {
        assert_eq!(mutability(), 6);
    }

    #[test]
    fn test_shadowing() {
        assert_eq!(shadowing(), "twenty");
    }

    #[test]
    fn test_tuple_extract() {
        assert_eq!(tuple_extract((1, 2.5, 3, true, 'z')), (2.5, true));
    }

    #[test]
    fn test_sum_slice() {
        assert_eq!(sum_slice(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(sum_slice(&[]), 0);
    }

    #[test]
    fn test_max_of() {
        assert_eq!(max_of(3, 7), 7);
        assert_eq!(max_of(10, 2), 10);
        assert_eq!(max_of(5, 5), 5);
    }

    #[test]
    fn test_next_power_of_two() {
        assert_eq!(next_power_of_two(1), 1);
        assert_eq!(next_power_of_two(5), 8);
        assert_eq!(next_power_of_two(16), 16);
        assert_eq!(next_power_of_two(17), 32);
    }

    #[test]
    fn test_fizzbuzz() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }
}
