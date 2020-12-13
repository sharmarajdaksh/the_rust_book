# Advanced Functions and Closures

## Function pointers

Functions coerce to the type `fn` (with a lowercase f), not to be confused with the `Fn` closure trait.
The fn type is called a function pointer

```rust
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), so you can always
pass a function pointer as an argument for a function that expects a closure.
It’s best to write functions using a generic type and one of the closure traits so your
functions can accept either functions or closures.

An example of where you would want to only accept fn and not closures is when interfacing with external code that
doesn’t have closures: C functions can accept functions as arguments, but C doesn’t have closures.

As an example of where you could use either a closure defined inline or a named function, let’s look at a use of map.
To use the map function to turn a vector of numbers into a vector of strings, we could use a closure, like this:

```rust
fn main() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();
}
```

Using a function instread of a closure:

```rust
fn main() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
}
```

Note that we must use fully qualified syntax because there may be multiple available functions named `to_string`.

We have another useful pattern that exploits an implementation detail of tuple structs and tuple-struct enum variants.
These types use () as initializer syntax, which looks like a function call.
The initializers are actually implemented as functions returning an instance that’s constructed from their arguments.
We can use these initializer functions as function pointers that implement the closure traits,
which means we can specify the initializer functions as arguments for methods that take closures, like so:

```rust
fn main() {
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
```

Hwere we create `Status::Value` instances using each `u32` value in the range that `map` is called on by using
the initalizer of `Status::Value`.

## Returning Closures

Closures are represented by traits, which means you can't return closures directly.
In most cases where you might want to return a trait, you can use the concrete type that implements the trait as
the return value of the function.
This can't be done by closures because they don't have a concrete type that is returnable. You cannot use
the function pointer `fn` as a return type, for example. The following does not compile:

```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
```

The error references the Sized trait!
Rust doesn’t know how much space it will need to store the closure.
We saw a solution to this problem earlier. We can use a trait object.
Following code compiles okay:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```
