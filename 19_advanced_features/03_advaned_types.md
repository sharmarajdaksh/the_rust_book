# Advanced Types

## The Newtype Pattern for Tpe Safety and Abstraction

The newtype pattern is useful for tasks beyond those we’ve discussed so far,
including statically enforcing that values are never confused and indicating the units of a value.

Another use of the newtype pattern is in abstracting away some implementation details of a type:
the new type can expose a public API that is different from the API of the private inner type if we
used the new type directly to restrict the available functionality, for example.

Newtypes can also hide internal implementation.
For example, we could provide a People type to wrap a HashMap<i32, String> that stores a person’s ID associated with their name.
Code using People would only interact with the public API we provide, such as a method to add a name string to the
People collection; that code wouldn’t need to know that we assign an i32 ID to names internally.
The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details.

## Creating Type Synonyms with Type Aliases

Along with the newtype pattern, Rust provides the ability to declare a type alias to give an existing type another name.
For this we use the type keyword.

```rust
type Kilomerters = i32;
```

The main use case for type synonyms is to reduce repetition. For example, we might have a lengthy type like this:

```rust
Box<dyn Fn() + Send + 'static>
```

A type alias would make using this type more manageable.

```rust
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

Type aliases are also commonly used with the Result<T, E> type for reducing repetition.
Consider the std::io module in the standard library.
I/O operations often return a Result<T, E> to handle situations when operations fail to work.
This library has a std::io::Error struct that represents all possible I/O errors.
Many of the functions in std::io will be returning Result<T, E> where the E is std::io::Error,
such as these functions in the Write trait:

```rust

#![allow(unused)]
fn main() {
use std::fmt;
use std::io::Error;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}
}
```

The `Result<..., Error>` type is repeated a lot, so the std::io has the type of alias declaration.

```rust
type Result<T> = std::result::Result<T, std::io::Error>;
```

The `Write` trait function signatures thus end up looking like this:

```rust
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

The type alias helps in two ways: it makes code easier to write and it gives us a consistent interface across all of std::io.
Because it’s an alias, it’s just another Result<T, E>, which means we can use any methods that work on Result<T, E>
with it, as well as special syntax like the ? operator.

## The Never Type that Never Returns

Rust has a special type named `!` that's known in type theory lingo as the empty type because it has no values.
Rust calles it _never type_ because it stands in the place of the return type when a function will never return.

```rust
fn bar() -> ! { // function bar returns never
    // --snip--
}
```

We can't create values fo the type `!` so bar can never possibly return.

Since match arms must all return the same type, we cannot have something like this:

```rust
let guess = match guess.trim().parse() {
    Ok(_) => 5,
    Err(_) => "hello",
};
```

This, however is valid. How?

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

The type of guess in this code would have to be an integer and a string, and Rust requires that guess have only one type.
So what does continue return? How were we allowed to return a u32 from one arm and have another arm that ends with continue.

`continue` has a `!` value. When Rust computes the type of guess, it looks at both match arms,
the former with a value of u32 and the latter with a ! value.
Because ! can never have a value, Rust decides that the type of guess is u32.

The formal way of describing this behavior is that expressions of type ! can be coerced into any other type.
We’re allowed to end this match arm with continue because continue doesn’t return a value;
instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.

The never type is useful with the panic! macro as well.
The `unwrap` function, for example, is defined like so:

```rust
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T.
This code works because panic! doesn’t produce a value; it ends the program.
In the None case, we won’t be returning a value from unwrap, so this code is valid.

One final expression that has the type `!` is a loop:

```rust
    print!("forever ");

    loop {
        print!("and ever ");
    }
```

## Dybanically sized types and the sized trait

Dynamically sized types or DSTs or unsized types allow us to write code using values whose size we can
only know at runtine. One of the most common DSTs is the `str` type.
We can’t know how long the string is until runtime, meaning we can’t create a variable of type `str`,
nor can we take an argument of type str. This code does not compile:

```rust
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
```

Rust needs to know how much memory to allocate for any value of a particular type,
and all values of a type must use the same amount of memory.
If Rust allowed us to write this code, these two str values would need to take up the same amount of space.
But they have different lengths: s1 needs 12 bytes of storage and s2 needs 15.
This is why it’s not possible to create a variable holding a dynamically sized type.
We thus use `&str` as our type instead.

So although a &T is a single value that stores the memory address of where the T is located, a &str is two values:
the address of the str and its length. As such, we can know the size of a &str value at compile time:
it’s twice the length of a usize. That is, we always know the size of a &str,
no matter how long the string it refers to is.
In general, this is the way in which dynamically sized types are used in Rust:
they have an extra bit of metadata that stores the size of the dynamic information.
The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

We can combine str with all kinds of pointers: for example, Box<str> or Rc<str>.
In fact, you’ve seen this before but with a different dynamically sized type: traits.
Every trait is a dynamically sized type we can refer to by using the name of the trait.

To work with DSTs, Rust has a particular trait called the Sized trait to determine whether or not
a type’s size is known at compile time. This trait is automatically implemented for everything whose
size is known at compile time. In addition, Rust implicitly adds a bound on Sized to every generic function.
That is, a generic function definition like this:

```rust
fn generic<T>(t: T) {
    // code
}
```

is treated like this

```rust
fn generic<T: Sized>(t: T) {
    // --snip--
}
```

By default, generic functions will work only on types that have a known size at compile time.
However, you can use the following special syntax to relax this restriction:

```rust
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

A trait bound on `?Sized` is the opposite of a trait bound on `Sized`:
we would read this as “`T` may or may not be `Sized`.”
This syntax is only available for `Sized`, not any other traits.

Also note that we switched the type of the `t` parameter from `T` to `&T`.
Because the type might not be Sized, we need to use it behind some kind of pointer.
In this case, we’ve chosen a reference.
