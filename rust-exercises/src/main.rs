// Rust Book Exercise Suite
// =======================
// Run individual exercises with: cargo test e01 (or e02, e03, etc.)
// Run all exercises with: cargo test
// Exercises are ordered by chapter — work through them sequentially.
//
// Each exercise has a TODO comment. Replace the todo!() macros with
// working code. Tests at the bottom of each file verify correctness.
//
// Chapters covered:
//   e01 - Variables, types, functions, control flow (Ch 3)
//   e02 - Ownership, borrowing, slices (Ch 4)
//   e03 - Structs, enums, pattern matching (Ch 5-6)
//   e04 - Collections and error handling (Ch 8-9)
//   e05 - Generics and traits (Ch 10.1-10.2)
//   e06 - Lifetimes (Ch 10.3)
//   e07 - Closures and iterators (Ch 13)
//   e08 - Smart pointers (Ch 15)
//   e09 - Concurrency (Ch 16)
//   e10 - Advanced features (Ch 19-20)

mod e01_basics;
mod e02_ownership;
mod e03_structs_enums;
mod e04_collections_errors;
mod e05_generics_traits;
mod e06_lifetimes;
mod e07_closures_iterators;
mod e08_smart_pointers;
mod e09_concurrency;
mod e10_advanced;

fn main() {
    println!("Run `cargo test` to check your exercises!");
    println!("Run `cargo test e01` to test a specific module.");
}
