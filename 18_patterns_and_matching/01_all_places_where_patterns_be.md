# All places where patterns be

## match Arms

Formally, `match` expressions are defined as the keyword `match`, a value to match on, and one or more match arms
that consist of a pattern and an expression to run if the value matches the arm's pattern.

```rust
match VALUE {
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
}
```

One requirement for `match` is that they need to be _exhaustive_. One way to ensure this is to have a catchall pattern
in the last arm.

The pattern `_` will match anythign, but it never binds to a variable, so is often used in the last match arm.
The `_` pattern can be useful when you want to ignore any value not specified.

## Conditional if let Expressions

`if let` are a shorter way to write match with single case, with an optional `else` with it if the `if let` doesn't match.
It's also possible to match `if let`, `else if`, and `else if let` expressions.
Conditions in a series of `if let`, `else if`, `else if let` arms aren't required to be related to each other.

```rust
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

## while let Conditional Loops

`while let` allows loops to run for as long as a pattern continues to match.

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

## for Loops

`for` loop is the most common loop in Rust. In a `for` loop, the pattern in the value that directly follows the keyword `for`,
so `for x in y` sees `x` as the pattern.

```rust
let v = vec!['a', 'b', 'c'];

for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
}
```

We use the enumerate method to adapt an iterator to produce a value and that value’s index in the iterator, placed into a tuple.
The first call to enumerate produces the tuple (0, 'a'). When this value is matched to the pattern (index, value), index will be
0 and value will be 'a', printing the first line of the output.

## let statements

```rust
let x = 5;
let (x, y, z) = (1, 2, 3);
```

# Function Parameters

```rust
fn foo(x: i32) {
    // code
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

Patterns can ve used in closure paramter lists in the same way as in function parameter lists.
