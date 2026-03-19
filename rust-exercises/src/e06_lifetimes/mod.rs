// ============================================================
// E06: Lifetimes (Ch 10.3)
// ============================================================

// --- Exercise 1: Basic lifetime annotation ---
// Fix the signature so this compiles. Return the longer of two string slices.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO: Add lifetime annotations to make this compile.
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

// --- Exercise 2: Lifetime with one relevant parameter ---
// This function only ever returns `x`. Fix the signature minimally.
pub fn first_only<'a>(x: &'a str, _y: &str) -> &'a str {
    // TODO: What's the minimal lifetime annotation needed?
    x
}

// --- Exercise 3: Struct with lifetime ---
// Define a struct `Excerpt` that borrows a string slice.
// It should have a field `text: &str` with appropriate lifetime.

pub struct Excerpt<'a> {
    pub text: &'a str,
}

impl<'a> Excerpt<'a> {
    // TODO: Return the number of words in the excerpt.
    pub fn word_count(&self) -> usize {
        todo!()
    }
}

// --- Exercise 4: Multiple lifetimes ---
// This struct holds two references that may have different lifetimes.
// Define it so that `first` and `second` can have independent lifetimes.

pub struct TwoRefs<'a, 'b> {
    pub first: &'a str,
    pub second: &'b str,
}

// Return just the first reference — the return lifetime should only
// depend on 'a.
pub fn get_first<'a, 'b>(refs: &TwoRefs<'a, 'b>) -> &'a str {
    // TODO
    todo!()
}

// --- Exercise 5: Lifetime elision ---
// These functions compile without explicit lifetimes due to elision rules.
// For each, write a comment explaining WHICH elision rule applies.

// Rule: ___
pub fn first_char(s: &str) -> &str {
    &s[..1]
}

// Rule: ___
pub fn identity(s: &str) -> &str {
    s
}

// --- Exercise 6: Static lifetime ---
// Return a string slice with 'static lifetime.
pub fn static_greeting() -> &'static str {
    // TODO: Return a string literal (which has 'static lifetime).
    todo!()
}

// --- Exercise 7: Lifetime in method ---
// Implement a method on Excerpt that returns the longer of its text
// and another &str.
impl<'a> Excerpt<'a> {
    pub fn longer_than(&self, other: &'a str) -> &'a str {
        // TODO
        todo!()
    }
}

// --- Exercise 8: Combining lifetimes, generics, and trait bounds ---
// Write a function with a lifetime parameter, a generic type T: Display,
// and return the longer of x and y after printing ann.
use std::fmt::Display;

pub fn longest_with_announcement<'a, T: Display>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str {
    // TODO: Print the announcement, then return the longer string.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("short", "looooong"), "looooong");
        assert_eq!(longest("equal", "sizes"), "equal"); // same len => first
    }

    #[test]
    fn test_first_only() {
        let result;
        {
            let x = String::from("hello");
            let y = String::from("y");
            result = first_only(&x, &y);
            assert_eq!(result, "hello");
        }
    }

    #[test]
    fn test_excerpt() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let exc = Excerpt { text: &novel[..16] };
        assert_eq!(exc.word_count(), 3);
    }

    #[test]
    fn test_two_refs() {
        let a = String::from("alpha");
        let b = String::from("beta");
        let tr = TwoRefs { first: &a, second: &b };
        assert_eq!(get_first(&tr), "alpha");
    }

    #[test]
    fn test_static_greeting() {
        let s: &'static str = static_greeting();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_longer_than() {
        let novel = String::from("hello world");
        let exc = Excerpt { text: &novel };
        assert_eq!(exc.longer_than("hi"), "hello world");
    }

    #[test]
    fn test_longest_with_announcement() {
        let result = longest_with_announcement("short", "much longer", "Comparing!");
        assert_eq!(result, "much longer");
    }
}
