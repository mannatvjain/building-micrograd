// ============================================================
// E02: Ownership, Borrowing, and Slices (Ch 4)
// ============================================================

// --- Exercise 1: Move semantics ---
// Fix this function so it compiles WITHOUT cloning.
// Hint: Think about what `takes_ownership` should accept.
pub fn move_semantics() -> String {
    let s = String::from("hello");
    prints_string(&s); // TODO: fix the function signature below
    s // we still need s here
}

fn prints_string(s: &String) {
    println!("{s}");
}

// --- Exercise 2: Clone ---
// Sometimes you DO want a deep copy. Make two independent Strings.
pub fn clone_example() -> (String, String) {
    let s1 = String::from("hello");
    // TODO: Create s2 as an independent copy of s1.
    // Return (s1, s2) — both must be valid.
    todo!()
}

// --- Exercise 3: Mutable references ---
// Write a function that appends " world" to the given String.
pub fn append_world(s: &mut String) {
    // TODO
    todo!()
}

// --- Exercise 4: Multiple references ---
// Fix this function so it compiles. You cannot change the assertions.
pub fn reference_rules() -> (String, usize) {
    let mut s = String::from("hello");

    // TODO: This won't compile as-is. Restructure so that the
    // immutable borrow is finished before the mutable borrow begins.
    let r1 = &s;
    let len = r1.len();
    // We want to push to s after using r1
    s.push_str(" world");

    (s, len)
}

// --- Exercise 5: String slices ---
// Return the first word of a string (up to the first space, or the
// whole string if no space). Return a &str slice, not a new String.
pub fn first_word(s: &str) -> &str {
    // TODO
    todo!()
}

// --- Exercise 6: Slice patterns ---
// Given a slice of integers, return a slice containing only the
// middle element(s). For odd length, return the single middle element.
// For even length, return the two middle elements.
pub fn middle_elements(s: &[i32]) -> &[i32] {
    // TODO
    todo!()
}

// --- Exercise 7: Ownership puzzle ---
// This function should take a vector, add 10 to each element,
// and return the modified vector. Do NOT clone.
pub fn add_ten_to_each(mut v: Vec<i32>) -> Vec<i32> {
    // TODO: modify v in place and return it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_semantics() {
        assert_eq!(move_semantics(), "hello");
    }

    #[test]
    fn test_clone_example() {
        let (s1, s2) = clone_example();
        assert_eq!(s1, "hello");
        assert_eq!(s2, "hello");
    }

    #[test]
    fn test_append_world() {
        let mut s = String::from("hello");
        append_world(&mut s);
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_reference_rules() {
        let (s, len) = reference_rules();
        assert_eq!(s, "hello world");
        assert_eq!(len, 5);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
        assert_eq!(first_word(""), "");
    }

    #[test]
    fn test_middle_elements() {
        assert_eq!(middle_elements(&[1, 2, 3]), &[2]);
        assert_eq!(middle_elements(&[1, 2, 3, 4]), &[2, 3]);
        assert_eq!(middle_elements(&[42]), &[42]);
    }

    #[test]
    fn test_add_ten_to_each() {
        assert_eq!(add_ten_to_each(vec![1, 2, 3]), vec![11, 12, 13]);
    }
}
