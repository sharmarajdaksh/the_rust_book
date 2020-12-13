# Macros

_Macro_ refers to a family of Rust features: _declarative macros_ with `macro_rules!` and three kinds of _procedural_ macros:

- Custom `#[derive]` macros that specify code added with the derive attribute used on structs and enums
- Attribute-like macros that define custom attributes usable on any item
- Function-like macros that look like function calls but operate on the tokens specified as their argument

## Difference between macros and functions

Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.

Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions. However, macros have some additional powers that functions don’t.

A function signature must declare the number and type of parameters the function has.
Macros, on the other hand, can take a variable number of parameters:
we can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments.
Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can,
for example, implement a trait on a given type. A function can’t, because it gets called at
runtime and a trait needs to be implemented at compile time.

The downside to implementing a macro instead of a function is that macro definitions are more complex
than function definitions because you’re writing Rust code that writes Rust code.
Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions.

Another important difference between macros and functions is that you must define macros or bring them into
scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

## Declarative Macros with `macro_rules!` for General Metaprogramming

Declarative macros allows you to write something similar to a match expression. Macros compare a value to patterns
that are associated with particular code and when matched, the corresponding code replaces the code passed to the macro.
This happens during compilation.

To define a macro, you use the macro_rules! construct. Let’s explore how to use macro_rules! by looking at how the vec! macro is defined.
A simplified implementation of the `vec!` macro may be defined as:

```rust
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The `#[macro_export]` annotation indicates that this macro should be made available whenever the crate
in which the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.

We then start the macro definition with `macro_rules!` and the name of the macro we’re defining without the exclamation mark.
The name, in this case vec, is followed by curly brackets denoting the body of the macro definition.

The structure in the vec! body is similar to the structure of a match expression.
Here we have one arm with the pattern `( $( $x:expr ),* )`, followed by `=>` and the block of code associated with this pattern.
If the pattern matches, the associated block of code will be emitted.
Given that this is the only pattern in this macro, there is only one valid way to match;
any other pattern will result in an error. More complex macros will have more than one arm.

Valid pattern syntax in macro definitions is different than the pattern syntax.

First, a set of parentheses encompasses the whole pattern.
A dollar sign (`$`) is next, followed by a set of parentheses that captures values that match the pattern
within the parentheses for use in the replacement code.
Within `$()` is `$x:expr`, which matches any Rust expression and gives the expression the name `$x`.

The comma following `$()` indicates that a literal comma separator character could optionally
appear after the code that matches the code in `$()`.
The `*` specifies that the pattern matches zero or more of whatever precedes the `*`.

When we call this macro with `vec![1, 2, 3];`, the $x pattern matches three times with the three expressions 1, 2, and 3.

`temp_vec.push()` within `$()*` is generated for each part that matches `$()` in the pattern zero or more times
depending on how many times the pattern matches.
The `$x` is replaced with each expression matched.
When we call this macro with vec![1, 2, 3];, the code generated that replaces this macro call will be the following:

```rust
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

There are some strange edge cases with `macro_rules!`.
In the future, Rust will have a second kind of declarative macro that will work in a similar
fashion but fix some of these edge cases.
After that update, `macro_rules!` will be effectively deprecated.
With this in mind, as well as the fact that most Rust programmers will use macros more than write macros,
we won’t discuss macro_rules! any further.

## Procedural Macros for Generating Code from Attributes

Procedural macros act more like functions. They accept some code as input, operate on that code, and produce
some code as output rather than matching against patterns and replacing the code with other code like declarative macros.
Custom derive, attribute-like, and function-like, all three types work similarly.

When creating procedural macros, the definitions must reside in their own crate with a special crate type.
This is for complex technical reasons that we hope to eliminate in the future.
Using procedural macros looks like the code below, where `some_attribute` is a placeholder for using a specific macro.

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

The function that defines a procedural macro takes a `TokenStream` as an input and produces a `TokenStream` as an output.
The `TokenStream` type is defined by the `proc_macro` crate that is included with Rust and represents a sequence of tokens.
This is the core of the macro: the source code that the macro is operating on makes up the input `TokenStream`,
and the code the macro produces is the output `TokenStream`.
The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating.
We can have multiple kinds of procedural macros in the same crate.

### Writing a custom derive macro

This topic is, urm... too complicated to condense. And perhaps, irrelevant as of now, since it needs to be
better implemented, as the documentation itself concedes.

### Attribute-like macros

These are like custom derive macros but instead of generating code for the derive attribute, they allow you to create new attributes.
These are also more flexible, as `derive` only works for structs and enums, but these can be applied to other items like functions as well.

In a function framework, you might see something like:

```rust
#[route(GET, "/")]
fn index() {
```

The signature of the macro definition may look something like this:

```rust
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```

## Function-like macros

Function-like macros define macros that look like function calls. Similarly to `macro_rules!` macros,
they’re more flexible than functions; for example, they can take an unknown number of arguments.
However, `macro_rules!` macros can be defined only using the match-like syntax.

Function-like macros take a `TokenStream` parameter and their definition manipulates that `TokenStream` using Rust code as the other
two types of procedural macros do. An example of a function-like macro is an `sql!` macro that might be called like so:

```rust
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

This macro would parse the SQL statement inside it and check that it’s syntactically correct,
which is much more complex processing than a `macro_rules!` macro can do. The `sql!` macro would be defined like this:

```rust
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

This definition is similar to the custom derive macro’s signature:
we receive the tokens that are inside the parentheses and return the code we wanted to generate.
