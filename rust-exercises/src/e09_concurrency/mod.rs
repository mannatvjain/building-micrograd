// ============================================================
// E09: Concurrency (Ch 16)
// ============================================================
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// --- Exercise 1: Spawning threads ---
// Spawn 5 threads, each printing its index. Collect all handles and
// join them. Return the thread count that completed successfully.
pub fn spawn_and_join() -> usize {
    // TODO: Spawn 5 threads, join them all, count successes.
    todo!()
}

// --- Exercise 2: Move closures ---
// Spawn a thread that takes ownership of a vector, sums it, and
// returns the sum via the JoinHandle.
pub fn thread_sum(nums: Vec<i32>) -> i32 {
    // TODO: Use `move` closure to transfer ownership to the thread.
    todo!()
}

// --- Exercise 3: Message passing with channels ---
// Spawn a thread that sends numbers 1..=n through a channel.
// The main thread collects them into a Vec.
pub fn channel_collect(n: i32) -> Vec<i32> {
    // TODO: Create a channel, spawn a sender thread, collect in main thread.
    todo!()
}

// --- Exercise 4: Multiple producers ---
// Spawn `n` threads, each sending its thread ID through the channel.
// Return a sorted vec of received IDs.
pub fn multi_producer(n: usize) -> Vec<usize> {
    // TODO: Clone the sender for each thread.
    todo!()
}

// --- Exercise 5: Shared state with Mutex ---
// Spawn 10 threads, each incrementing a shared counter 100 times.
// Return the final count.
pub fn mutex_counter() -> i32 {
    // TODO: Use Arc<Mutex<i32>> to share the counter across threads.
    todo!()
}

// --- Exercise 6: Arc<Mutex<Vec<T>>> ---
// Spawn n threads. Each thread pushes its ID squared into a shared vec.
// Return the vec sorted.
pub fn parallel_squares(n: usize) -> Vec<usize> {
    // TODO
    todo!()
}

// --- Exercise 7: Deadlock avoidance ---
// Given two mutexes, acquire them in consistent order to avoid deadlock.
// Return the sum of both values.
pub fn safe_sum(a: &Mutex<i32>, b: &Mutex<i32>) -> i32 {
    // TODO: Always lock a before b (or use try_lock patterns).
    // This exercises awareness of deadlock, not a complex solution.
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_and_join() {
        assert_eq!(spawn_and_join(), 5);
    }

    #[test]
    fn test_thread_sum() {
        assert_eq!(thread_sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_channel_collect() {
        assert_eq!(channel_collect(5), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_mutex_counter() {
        assert_eq!(mutex_counter(), 1000);
    }

    #[test]
    fn test_parallel_squares() {
        let result = parallel_squares(5);
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_safe_sum() {
        let a = Mutex::new(10);
        let b = Mutex::new(20);
        assert_eq!(safe_sum(&a, &b), 30);
    }
}
