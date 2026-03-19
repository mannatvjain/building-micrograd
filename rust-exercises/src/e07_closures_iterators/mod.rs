// ============================================================
// E07: Closures and Iterators (Ch 13)
// ============================================================

// --- Exercise 1: Basic closure ---
// Store a closure in a variable that doubles its input.
pub fn double_with_closure() -> Vec<i32> {
    // TODO: Define a closure `double` that takes an i32 and returns i32 * 2.
    // Then use it to map over the vec.
    let nums = vec![1, 2, 3, 4, 5];
    todo!()
}

// --- Exercise 2: Closure capturing ---
// Write a function that returns a closure which adds `n` to its argument.
// The closure must own `n`.
pub fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // TODO: Return a closure that captures n by value.
    move |x| { let _ = n; todo!() }
}

// --- Exercise 3: FnMut closure ---
// Use a closure to accumulate a running total while iterating.
pub fn running_total(nums: &[i32]) -> Vec<i32> {
    // TODO: Return a vec where each element is the cumulative sum.
    // e.g., [1, 2, 3] => [1, 3, 6]
    // Hint: Use a mutable variable captured by a closure with map.
    todo!()
}

// --- Exercise 4: Iterator adaptors ---
// Chain multiple iterator adaptors: filter, map, and collect.
// Given a list of strings, return the lengths of strings that start with 'a'.
pub fn a_word_lengths(words: &[&str]) -> Vec<usize> {
    // TODO: filter words starting with 'a', map to length, collect.
    todo!()
}

// --- Exercise 5: fold / reduce ---
// Implement factorial using Iterator::fold.
pub fn factorial(n: u64) -> u64 {
    // TODO: Use (1..=n).fold(...)
    todo!()
}

// --- Exercise 6: Custom iterator ---
// Implement an iterator that yields Fibonacci numbers.
pub struct Fibonacci {
    // TODO: Define the fields (current and next values).
}

impl Fibonacci {
    pub fn new() -> Self {
        // TODO
        todo!()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: Yield the current value, advance state.
        todo!()
    }
}

// --- Exercise 7: enumerate and zip ---
// Given two slices, return a vec of tuples (index, a_elem, b_elem)
// for positions where elements differ.
pub fn diff_positions(a: &[i32], b: &[i32]) -> Vec<(usize, i32, i32)> {
    // TODO: Use zip and enumerate.
    todo!()
}

// --- Exercise 8: Chaining iterators ---
// Flatten a Vec<Vec<i32>> into a single sorted, deduplicated Vec<i32>.
pub fn flatten_sort_dedup(nested: Vec<Vec<i32>>) -> Vec<i32> {
    // TODO: Use into_iter, flatten, collect, sort, dedup.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_with_closure() {
        assert_eq!(double_with_closure(), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(3), 8);
        assert_eq!(add5(10), 15);
    }

    #[test]
    fn test_running_total() {
        assert_eq!(running_total(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
        assert_eq!(running_total(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_a_word_lengths() {
        let words = vec!["apple", "banana", "avocado", "cherry", "apricot"];
        assert_eq!(a_word_lengths(&words), vec![5, 7, 7]);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(8).collect();
        assert_eq!(fibs, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_diff_positions() {
        let a = vec![1, 2, 3, 4];
        let b = vec![1, 9, 3, 7];
        assert_eq!(diff_positions(&a, &b), vec![(1, 2, 9), (3, 4, 7)]);
    }

    #[test]
    fn test_flatten_sort_dedup() {
        let nested = vec![vec![3, 1, 2], vec![2, 4, 1], vec![5, 3]];
        assert_eq!(flatten_sort_dedup(nested), vec![1, 2, 3, 4, 5]);
    }
}
