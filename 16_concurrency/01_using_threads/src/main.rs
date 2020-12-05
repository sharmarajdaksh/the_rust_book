// Basic problems of concurrency: race conditions and deadlocks

// Programming languages implement threads in a few different ways.
// Many operating systems provide an API for creating new threads.
// This model where a language calls the operating system APIs to create
// threads is sometimes called 1:1, meaning one operating system thread
// per one language thread.
// Many programming languages provide their own special implementation of
// threads.
//  Programming language-provided threads are known as green threads, and
// languages that use these green threads will execute them in the context
// of a different number of operating system threads.
// For this reason, the green-threaded model is called the M:N model:
// there are M green threads per N operating system threads, where
// M and N are not necessarily the same number.

// Each model has its own advantages and trade-offs, and the trade-off most
// important to Rust is runtime support.
// By runtime we mean code that is included by the language in every binary.
// This code can be large or small depending on the language, but every
// non-assembly language will have some amount of runtime code.
// Although many languages are okay with increasing the runtime size in
// exchange for more features, Rust needs to have nearly no runtime and
// cannot compromise on being able to call into C to maintain performance.
// The green-threading M:N model requires a larger language runtime to manage
// threads.
// As such, the Rust standard library only provides an implementation of 1:1
// threading.
// Because Rust is such a low-level language, there are crates that implement
// M:N threading if you would rather trade overhead for aspects such as more
// control over which threads run when and lower costs of context switching

use std::thread;
use std::time::Duration;

fn main() {
    // Spawning threads without waiting on them to complete:
    //
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("number {} from the spawned thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    //

    // We can fix the problem of the spawned thread not getting to run, or not
    // getting to run completely, by saving the return value of thread::spawn
    // in a variable. The return type of thread::spawn is JoinHandle.
    // A JoinHandle is an owned value that, when we call the join method on it,
    // will wait for its thread to finish
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join() blocks the main thread until the threads it itself represents
    // terminate
    handle.join().unwrap();

    // `move` closures are used to allow using data from one thread in another
    // The move keyword can be used before the parameter list of a closure to
    // force the closure to take ownership of values it uses in the environment.
    // This is especially useful in new threads to transfer ownership.
    // To use data from the main thread in a spawned thread, the spawned
    // thread's closure must capture the values it needs

    let v = vec![1, 2, 3];
    // Rust infers how to capture v, and because println! only needs a
    // reference to v, the closure tries to borrow v.
    // However, there’s a problem: Rust can’t tell how long the spawned
    // thread will run, so it doesn’t know if the reference to v will
    // always be valid.
    // By adding the move keyword before the closure, we force the closure
    // to take ownership of the values it’s using rather than allowing Rust
    // to infer that it should borrow the values

    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    // drop(v); // invalid, since the closure took ownership

    handle.join().unwrap();
}
