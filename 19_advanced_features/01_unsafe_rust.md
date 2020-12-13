# Unsafe Rust

Unsafe Rust exists because, by nature, static analysis is conservative.
When the compiler tries to determine whether or not code upholds the guarantees, it’s better for it to
reject some valid programs rather than accept some invalid programs. Although the code might be okay,
as far as Rust is able to tell, it’s not!
In these cases, you can use unsafe code to tell the compiler,
“Trust me, I know what I’m doing.”
The downside is that you use it at your own risk:
if you use unsafe code incorrectly, problems due to memory unsafety, such as null pointer dereferencing, can occur.

Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.
If Rust didn’t let you do unsafe operations, you couldn’t do certain tasks. Rust needs to allow you to do
low-level systems programming, such as directly interacting with the operating system or even writing your own
operating system. Working with low-level systems programming is one of the goals of the language.

## Unsafe abilities

To switch to unsafe Rust, use the `unsafe` keywork, and start a new block holding the unsafe code.

Five so called _unsafe superpowers_ available in unsafe mode are:

- Deferencing a raw pointer
- Calling an unsafe function or method
- Access for modify a mutable static variable
- Implement an unsafe trait
- Access fields of `union`s.

`unsafe` doesn't turn off borrow checking or disable any other safety checks. It only enables
these five features that are then not checked by the compiler for memory safety.

People are fallible, and mistakes will happen, but by requiring these five unsafe operations to
be inside blocks annotated with unsafe you’ll know that any errors related to memory safety must be within an unsafe block.

To isolate unsafe code as much as possible, it’s best to enclose unsafe code within a safe abstraction and provide a safe API.
Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.
Wrapping unsafe code in a safe abstraction prevents uses of unsafe from leaking out into all the places that
you or your users might want to use the functionality implemented with unsafe code, because using a safe abstraction is safe.

## Dereferencing a Raw Pointer

Unsafe Rust has two new types called raw pointers that are similar to references.
As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
The asterisk isn’t the dereference operator; it’s part of the type name.
In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

- Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
- Aren’t guaranteed to point to valid memory
- Are allowed to be null
- Don’t implement any automatic cleanup

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}
```

We don’t include the unsafe keyword in this code. We can create raw pointers in safe code;
we just can’t dereference raw pointers outside an unsafe block.

We’ve created raw pointers by using as to cast an immutable and a mutable reference into their
corresponding raw pointer types. Because we created them directly from references guaranteed to be valid,
we know these particular raw pointers are valid, but we can’t make that assumption about just any raw pointer.

Next, we’ll create a raw pointer whose validity we can’t be so certain of.
We try to create a pointer to an arbitrary location in memory.
Trying to use arbitrary memory is undefined: there might be data at that address or there might not,
the compiler might optimize the code so there is no memory access,
or the program might error with a segmentation fault.
Usually, there is no good reason to write code like this, but it is possible.

```rust
let address = 0x012345usize;
let r = address as *const i32;
```

Recall that we can create raw pointers in safe code, but we can’t dereference raw pointers and read the data being pointed to.
Creating a pointer does no harm; it’s only when we try to access the value
that it points at that we might end up dealing with an invalid value.

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
```

Note also that we created *const i32 and *mut i32 raw pointers that both pointed to the same memory location,
where num is stored. If we instead tried to create an immutable and a mutable reference to num,
the code would not have compiled because Rust’s ownership rules don’t allow a mutable reference at the
same time as any immutable references. With raw pointers, we can create a mutable pointer and an immutable
pointer to the same location and change data through the mutable pointer, potentially creating a data race.

Some of the major use cases for raw pointers are:

- Interfacing with C code.
- Building up safe abstractions that the borrow checker can't understand.

## Calling an unsafe function or method

Unsafe functions and methods look exactly like regular functions and methods,
but they have an extra unsafe before the rest of the definition.
The unsafe keyword in this context indicates the function has requirements we need to
uphold when we call this function, because Rust can’t guarantee we’ve met these requirements.
By calling an unsafe function within an unsafe block, we’re saying that we’ve read this
function’s documentation and take responsibility for upholding the function’s contracts.
We must call the dangerous function within a separate unsafe block

```rust
unsafe fn dangerous() {}
fn main() {
    unsafe {
        dangerous();
    }

}
```

Bodies of unsafe functions are effectively unsafe blocks, so to perform other unsafe operations
within an unsafe function, we don’t need to add another unsafe block.

### Creating a safe abstraction over unsafe code

Wrapping unsafe code in a safe function is a common abstraction.
Consider the `split_at_mut` function from the standard library, that requires some unsafe code.
This safe method is defined on mutable slices: it takes one and makes it two by splitting
at the index given as argument.

Usage for `split_at_mut`:

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
```

We can’t implement this function using only safe Rust. An attempt like this won't compile:

```rust
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);

    (&mut slice[..mid], &mut slice[mid..])
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
```

Rust’s borrow checker can’t understand that we’re borrowing different parts of the slice;
it only knows that we’re borrowing from the same slice twice.
Borrowing different parts of a slice is fundamentally okay because the two slices aren’t overlapping,
but Rust isn’t smart enough to know this. When we know code is okay, but Rust doesn’t, it’s time to reach for unsafe code.

```rust
use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // Gives raw pointer of a slice *mut i32

    assert!(mid <= len);

    unsafe {
        (
            // from_raw_parts_mut takes a raw pointer and a length, and creates a slice
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
```

Note that we don’t need to mark the resulting split_at_mut function as unsafe,
and we can call this function from safe Rust.
We’ve created a safe abstraction to the unsafe code with an implementation of
the function that uses unsafe code in a safe way,
because it creates only valid pointers from the data this function has access to.

### Using extern Functions to call External Code

Rust's `extern` keyword facilitates the creation and use of a Foreign Function Interface (FFI)
to enable calling functions from code in another language.

Setting up an integration with the abs function from the C standard library:

```rust
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Within the `extern "C"` block, we list the names and signatures of external functions from another language we want to call.
The `"C"` part defines which application binary interface (ABI) the external function uses:
the ABI defines how to call the function at the assembly level.
The "C" ABI is the most common and follows the C programming language’s ABI.

Functions declared within extern blocks are always unsafe to call from Rust code. The reason is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.

### Calling rust from other languages

`extern` can also allow use to create an interface that allows other languages to call Rust functions.
Instead of an extern block, we add the extern keyword and specify the ABI to use just before the fn keyword.
We also need to add a `#[no_mangle]` annotation to tell the Rust compiler not to mangle the name of this function.
Mangling is when a compiler changes the name we’ve given a function to a different name that contains more
information for other parts of the compilation process to consume but is less human readable.
Every programming language compiler mangles names slightly differently,
so for a Rust function to be nameable by other languages, we must disable the Rust compiler’s name mangling.

```rust
#![allow(unused)]

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

## Accessing or modiying a mutable static variable

Rust supports global `static` variables but it can be problematic to manage with Rust's ownership rules.

```rust
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

The names of static variables are in SCREAMING_SNAKE_CASE by convention, and we must annotate the
variable’s type, which is &'static str in this example.
Static variables can only store references with the 'static lifetime,
which means the Rust compiler can figure out the lifetime;
we don’t need to annotate it explicitly. Accessing an immutable static variable is safe.

Constants and immutable static variables might seem similar, but a subtle difference is that
values in a static variable have a fixed address in memory. Using the value will always access the same data.
Constants, on the other hand, are allowed to duplicate their data whenever they’re used.

Another difference between constants and static variables is that static variables can be mutable.
Accessing and modifying mutable static variables is unsafe.

```rust
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

Any code that reads or writes from COUNTER must be within an unsafe block.

## Implementing an unsafe trait

A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
We can declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too.

```rust
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

As an example, recall the Sync and Send marker traits.
The compiler implements these traits automatically if our types are composed entirely of Send and Sync types.
If we implement a type that contains a type that is not Send or Sync, such as raw pointers,
and we want to mark that type as Send or Sync, we must use unsafe.
Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or
accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with unsafe.

## Accessing fields of a Union

A `union` allows only one declared field to be used for a particular instance at one time.
Unions are primarily used to interface with unions in C code.
Accessing union fields is unsafe because Rust can’t guarantee the
type of the data currently being stored in the union instance.
