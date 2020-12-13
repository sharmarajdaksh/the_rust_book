# Advanced Traits

## Placeholder types in Trait definitions with associated types

_Associated types_ connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
Trait implementors must specify the concrete types to be used in the type's place.
This allows defining traits using some types without needing to know exactly what those types are until the trait is implemented.

One example of a trait with an associated type is the `Iterator` trait that the standard library provides.
The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over.

The definition of the `Iterator` trait is like so:

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The type `Item` is a placeholder type, and the `next` method's definition shows that it returns values of type Option<Self::Item>.
Implementors specify the concrete type for `Item`, and the `next` method will return an `Option` containing the value of the concrete type.

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // code
    }
}
```

The approach might look like it could've just used Generics such that the Iterator trait became:

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is that when using generics, we could have multiple implementations of Iterator for Counter.
In other words, when a trait has a generic parameter, it can be implemented for a type multiple times,
changing the concrete types of the generic type parameters each time.
When we use the next method on Counter, we would have to provide type annotations
to indicate which implementation of Iterator we want to use.

With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.
For a type implementing `Iterator` we can only choose what the type of Item will be once, because there can only
be one impl Iterator block.
We don’t have to specify that we want an iterator of a particular type.

## Default Generic Type Parameters and Operator Overloading

When we use generic type parameters, we can specify a default concrete type for the generic type.
This eliminates the need for implementors of the trait to specify a concrete type if the default type works.
The syntax for specifying a default type for a generic type is `<PlaceholderType=ConcreteType>` when declaring the generic type.

A great example of a situation where this technique is useful is with operator overloading.

Rust doesn’t allow you to create your own operators or overload arbitrary operators.
But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator.

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // The Add trait has an associated type named Output that determines the type returned from the add method.
    type Output = Point;

    // we overload the + operator to add two Point instances together.
    // We do this by implementing the Add trait on a Point struct
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

The default generic type in this code is within the Add trait.

```rust
#![allow(unused)]
fn main() {
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
}
```

This code should look generally familiar: a trait with one method and an associated type.
The new part is Rhs=Self: this syntax is called _default type parameters_.
The Rhs generic type parameter defines the type of the rhs parameter in the add method.
If we don’t specify a concrete type for Rhs when we implement the Add trait,
the type of Rhs will default to Self, which will be the type we’re implementing Add on.

It might sometimes be useful to override the Rhs value in the Add trait for example. Say,
if you wanted to add Miliimteters and Meters structs

```rust
#![allow(unused)]
fn main() {
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
}
```

You’ll use default type parameters in two main ways:

- To extend a type without breaking existing code
- To allow customization in specific cases most users won’t need

## Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as another trait’s method,
nor does Rust prevent you from implementing both traits on one type.
It’s also possible to implement a method directly on the type with the same name as methods from traits.
When calling methods with the same name, you’ll need to tell Rust which one you want to use.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// When we call fly on an instance of Human, the compiler
// defaults to calling the method that is directly implemented on the type
fn main() {
    let person = Human;
    person.fly(); // calls Human.fly()

    // Calling fly methods from the Pilot or Wizard traits
    Pilot::fly(&person);
    Wizard::fly(&person);
}
```

Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
We could also write Human::fly(&person), which is equivalent to the person.fly()

However, associated functions that are part of traits don’t have a self parameter.
When two types in the same scope implement that trait, Rust can’t figure out which type you mean unless you use fully qualified syntax.

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name()); // calls the Dog::baby_name implementation "Spot"

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // Disambiguates that we want to use the impl
                                                                         // of Animal for Dog
                                                                         // Prints "puppy"
}
```

In general, fully qualified syntax is defined as follows:

```rust
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

For associated functions, there would not be a receiver: there would only be the list of other arguments.
You could use fully qualified syntax everywhere that you call functions or methods.
However, you’re allowed to omit any part of this syntax that Rust can figure out from other information in the program.
You only need to use this more verbose syntax in cases where there are multiple implementations that use the
same name and Rust needs help to identify which implementation you want to call.

## Using Supertraits to require One Trait's functionality within another

Sometimes, when you need one trait to use another trait's functionality. In this case, you need to rely on dependent traits also being implemented.
The trait you rely on is a _supertrait_ of the trait you're implementing.

For example, let’s say we want to make an OutlinePrint trait with an outline_print method that will print a value framed in asterisks.
That is, given a Point struct that implements Display to result in (x, y), when we call outline_print on a Point instance
that has 1 for x and 3 for y, it should print the following:

```
**********
*        *
* (1, 3) *
*        *
**********
```

In the implementation of outline_print, we want to use the Display trait’s functionality.
Therefore, we need to specify that the OutlinePrint trait will work only for types that also implement Display and
provide the functionality that OutlinePrint needs. We can do that in the trait definition by specifying OutlinePrint: Display.
This technique is similar to adding a trait bound to the trait.

```rust
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

fn main() {}
```

Because `OutlinePrint` requires the `Display` trait, we can use the `to_string` function that is automatically implemented for any type that implements
`Display`. If we tried to use `to_string` without specifying the `Display` trait after the trait name, we'd get an error.

If we try implementing `OutlinePrint` on a type that doesn't implement `Display` such as the `Point` struct, we'd get an error.
A type implementing `OutlinePrint` must also implement `Display`.

```rust
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

## Using the Newtype Pattern to Implement External Traits on External Types

The orphan rule states we’re allowed to implement a trait on a type as long as either the trait or the type are local to our crate.
It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.
The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for.
Then the wrapper type is local to our crate, and we can implement the trait on the wrapper.
There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.

As an example, let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly
because the Display trait and the Vec<T> type are defined outside our crate.
We can make a Wrapper struct that holds an instance of Vec<T>; then we can implement Display on Wrapper and use the Vec<T> value.

```rust
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

The implementation of Display uses self.0 to access the inner Vec<T>, because Wrapper is a tuple struct and Vec<T> is the item at
index 0 in the tuple. Then we can use the functionality of the Display type on Wrapper.

The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding.
We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0,
which would allow us to treat Wrapper exactly like a Vec<T>.
If we wanted the new type to have every method the inner type has, implementing the Deref trait.
If we don’t want the Wrapper type to have all the methods of the inner type—for example,
to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.
