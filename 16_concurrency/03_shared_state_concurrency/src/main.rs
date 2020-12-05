// In a way, channels enforce single ownership.
// Shared memory concurrency is like multiple ownership
//
// One way of managing shared resources are mutexes
// Mutexes require the use of the two rules:
// - You must attempt to acquire the lock before using the data
// - When you're done using the data, you must unlock the data
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    // counter is not an i32.
    // We must acquire a lock to be able to use the i32 value
    // The type system won't allow access to the inner i32 otherwise
    //
    // Mutex<T> is a smart pointer. More accurately, the call to lock() returns
    // a smart pointer called MutexGuard implementing Deref (pointing to inner
    // data) and Drop (releases the lock automatically when data goes out of
    // scope).

    let mut handles = vec![];

    for _ in 0..10 {
        // Rc<T> is unsafe for concurrent use, so we cannot do something like:
        // let counter = Rc::clone(&counter);
        // It doesn't implement the `Send` trait, necessary for concurrent use
        // So, instead we use Arc<T>, where the A is for atomic
        // Atomics work like primitive types but are safe to share across
        // threads
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            // Call to lock() blocks
            // Returns Err if the thread holding the lock panics
            // In case of a successful return, we can then use
            // `num` as a mutable reference to the data inside

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

// Similarities between Refcell<T>/Rc<T> and Mutex<T>/Arc<T>
//
// Mutex<T> provides interior mutability like the Cell family (counter is
// immutable, but not the value inside it); Mutex<T> can be used to mutate
// the contents inside an Arc<T> like RefCell<T> can mutate Rc<T> contents
//
// Another thing to know is that Rust can't catch all possible errors when
// using Mutex<T>. Like Rc<T> can potentially create reference cycles,
// Mutex<T> can potentially create deadlocks.
