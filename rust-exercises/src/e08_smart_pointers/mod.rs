// ============================================================
// E08: Smart Pointers (Ch 15)
// ============================================================
use std::cell::RefCell;
use std::rc::Rc;

// --- Exercise 1: Box<T> for recursive types ---
// Define a cons list using Box.
#[derive(Debug, PartialEq)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    // TODO: Construct a list from a slice. [1, 2, 3] => Cons(1, Cons(2, Cons(3, Nil)))
    pub fn from_slice(s: &[i32]) -> List {
        todo!()
    }

    // TODO: Convert back to a Vec.
    pub fn to_vec(&self) -> Vec<i32> {
        todo!()
    }
}

// --- Exercise 2: Custom smart pointer with Deref ---
// Implement a simple wrapper type that implements Deref.
use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        // TODO
        todo!()
    }
}

// --- Exercise 3: Drop trait ---
// Implement Drop for a type that records when it's dropped.
// Use a shared Vec<String> (via Rc<RefCell<Vec<String>>>) to log drops.
pub struct Droppable {
    pub name: String,
    pub log: Rc<RefCell<Vec<String>>>,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        // TODO: Push "{name} dropped" to the log.
        todo!()
    }
}

// --- Exercise 4: Rc<T> shared ownership ---
// Create a graph-like structure where multiple nodes share ownership
// of a child list.
pub fn shared_list() -> (Rc<List>, Rc<List>, Rc<List>) {
    // TODO:
    // Create a shared tail: Cons(5, Cons(10, Nil))
    // Create list_a: Cons(1, <shared tail>)
    // Create list_b: Cons(2, <shared tail>)
    // Return (shared_tail, list_a, list_b)
    // Hint: You'll need Rc<List> for the shared tail.
    todo!()
}

// --- Exercise 5: RefCell interior mutability ---
// Implement a simple counter that can be incremented through a shared reference.
pub struct Counter {
    value: RefCell<i32>,
}

impl Counter {
    pub fn new() -> Self {
        Counter { value: RefCell::new(0) }
    }

    // TODO: Increment the counter. Note: &self, not &mut self!
    pub fn increment(&self) {
        todo!()
    }

    pub fn get(&self) -> i32 {
        // TODO
        todo!()
    }
}

// --- Exercise 6: Rc<RefCell<T>> combination ---
// Model a shared mutable bank account that multiple owners can modify.
#[derive(Debug)]
pub struct BankAccount {
    balance: Rc<RefCell<f64>>,
}

impl BankAccount {
    pub fn new(initial: f64) -> Self {
        BankAccount {
            balance: Rc::new(RefCell::new(initial)),
        }
    }

    // TODO: Return a clone that shares the same balance.
    pub fn share(&self) -> BankAccount {
        todo!()
    }

    // TODO: Deposit amount.
    pub fn deposit(&self, amount: f64) {
        todo!()
    }

    // TODO: Withdraw amount. Return false if insufficient funds.
    pub fn withdraw(&self, amount: f64) -> bool {
        todo!()
    }

    pub fn balance(&self) -> f64 {
        // TODO
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cons_list() {
        let list = List::from_slice(&[1, 2, 3]);
        assert_eq!(list.to_vec(), vec![1, 2, 3]);
        assert_eq!(List::from_slice(&[]).to_vec(), Vec::<i32>::new());
    }

    #[test]
    fn test_my_box_deref() {
        let b = MyBox::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn test_drop_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        {
            let _a = Droppable { name: "a".into(), log: Rc::clone(&log) };
            let _b = Droppable { name: "b".into(), log: Rc::clone(&log) };
        } // b dropped first (reverse order), then a
        let log = log.borrow();
        assert_eq!(log[0], "b dropped");
        assert_eq!(log[1], "a dropped");
    }

    #[test]
    fn test_shared_list() {
        let (tail, a, b) = shared_list();
        assert_eq!(Rc::strong_count(&tail), 3); // tail + a + b
    }

    #[test]
    fn test_counter() {
        let c = Counter::new();
        c.increment();
        c.increment();
        c.increment();
        assert_eq!(c.get(), 3);
    }

    #[test]
    fn test_bank_account() {
        let acc1 = BankAccount::new(100.0);
        let acc2 = acc1.share();

        acc1.deposit(50.0);
        assert_eq!(acc2.balance(), 150.0); // shared!

        assert!(acc2.withdraw(30.0));
        assert_eq!(acc1.balance(), 120.0);

        assert!(!acc1.withdraw(200.0)); // insufficient
        assert_eq!(acc1.balance(), 120.0);
    }
}
