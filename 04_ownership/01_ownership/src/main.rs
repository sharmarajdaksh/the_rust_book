// Rules of Ownership in Rust:
// - Each value in Rust has a variable that’s called its owner.
// - There can only be one owner at a time.
// - When the owner goes out of scope, the value will be dropped.

fn main() {
    let s = "hello"; // s becomes valid when it comes into the scope
                     // It remains valid until it goes out of scope

    let mut s = String::from(s);
    s.push_str(", world!");
    // String type is allocated on the heap
    // - Memory must be requested from memory allocator at runtime
    // - Need a way to return memory to the allocator once the String's work is done

    // The String::from call requests the memory (at runtime)
    // For returning back memory, there is no Garbage Collector in Rust
    // The memory is automatically returned once the variable that owns it goes out of scope
    // At that point, Rust automatically calls `drop()`

    println!("{}", s);

    let s1 = String::from("hello");
    // String is a reference type (heap allocated). Right now, variable s1 points to heap
    let s2 = s1;
    // s2 now points to the same heap location/data as s1
    // However, at this point, s1 HAS LOST OWNERSHIP of the heap data
    // Rust considers s1 to no longer be valid

    // println!("{}", s1); // Won't worK!!

    // This passing of ownership from s1 to s2 is called a "move" in Rust

    // Of course it is also possible to deep copy (copy heap data as well) if needed
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} {}", s1, s2); // Valid since no move ever happened

    // Note that ownership is not really a problem for primitive (stack-allocated) types
    let x = 5;
    let y = x; // Copies the value itself, rather than the pointer
    println!("{} {}", x, y); // Valid

    // On the implementation side, types which have a special "Copy" trait are allowed
    // to be used even after assignment
    // Some of the types that are Copy:
    // - All integer, bool, float, char types
    // - Tuples, if they contain types that are also Copy
    //     - (i32, i32) will be Copy but now (i32, String)

    // Semantics of passing values to function are basically the same
    // Values might get copied or moved

    // Return values can also transfer ownership
    let s1 = gives_ownership();

    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);

    // Sometimes, however, you might want to pass data to functions without losing ownership
    // In come references
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
