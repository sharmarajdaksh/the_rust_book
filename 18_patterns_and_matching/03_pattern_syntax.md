# Pattern Syntax

## Matching Literals

```rust
let x = 1;

match x {
        1 => println!("one"), // matches
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
```

## Matching named variables

Named variables are irrefutable patterns that match any value. However, there is a problem using them in `match` expressions.
`match` starts a new scope, causing variables declared inside to shadow the ones outside.

```rust
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // matches. This y shadows the one outside
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
```

## Multiple Patterns

In `match`, multiple expressions can be matched in the same arm (OR) using `|`.

```rust
 let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

## Matching Ranges of Values with ..=

The `..=` syntax allows us to match to an inclusive range of values.

```rust
let x = 5;

match x {
    1..=5 => println!("1-5"), // matches
    _ => println!("something else"),
}
```

Ranges are only allowed with numeric values or char values, because the compiler checks that the range isn’t empty at compile time.
The only types for which Rust can tell if a range is empty or not are char and numeric values.

```rust
let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

## Destructuring to Break Apart Values

Patterns can also destructure structs, enums, tuples, and references to use different parts of these values.

### Destructuring structs

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

This code creates the variables a and b that match the values of the x and y fields of the p struct.
This example shows that the names of the variables in the pattern don’t have to match the field names of the struct.
But it’s common to want the variable names to match the field names to make it easier to remember which variables came from which fields.
So, doing this is also valid:

```rust
let Point { x, y } = p;
// Same as
let Point {x: x, y: y} = p;
```

We can also destructure with literal values as part of the struct pattern rather than creating variables for all the fields.
Doing so allows us to test some of the fields for particular values while creating variables to destructure the other fields.

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

### Destructuring Enums

The pattern to destructure an enum should correspond to the way the data stored within the enum is defined.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
    }
}
```

### Destructuring Nested Structs and Enums

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}
```

### Destructuring Structs and Tuples

Fairly complex destructuring is possible.

```rust
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}
```

## Ignoring Values in a Pattern

### Ignoring Entire value with \_

```rust
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}
```

Ignoring a function parameter can be especially useful in some cases, for example, when implementing a trait
when you need a certain type signature but the function body in your implementation doesn’t need one of the parameters.
The compiler will then not warn about unused function parameters, as it would if you used a name instead.

### Ignoring Parts of a Value with a Nested \_

```rust
// The business requirements are that the user should not be allowed to overwrite an existing customization of a
// setting but can unset the setting and give it a value if it is currently unset.
fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}
```

Multiple \_ can be used to ignore specific values

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {}, {}, {}", first, third, fifth)
    }
}
```

### Ignoring an unused variable with \_

If you create a variable but don’t use it anywhere, Rust will usually issue a warning because that could be a bug.
But sometimes it’s useful to create a variable you won’t use yet, such as when you’re prototyping or just starting a project.
In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore.

```rust
fn main() {
    let _x = 5;
}
```

Note that there is a subtle difference between using only _ and using a name that starts with an underscore.
The syntax \_x still binds the value to the variable, whereas _ doesn’t bind at all.
The following code will not compile:

```rust
fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
```

We’ll receive an error because the s value will still be moved into \_s, which prevents us from using s again.
However, using the underscore by itself doesn’t ever bind to the value.
The following code will work:

```rust
fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
```

### Ignoring Remaining Parts of a Value with ..

The `..` syntax can be used to make use of a few parts and ignore the rest, as if there were a variable
number of underscores specified.
`..` ignores any parts of a value that we haven't explicitly matched in the rest of the pattern.

```rust
fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x), // Ignore y and z
    }
}
```

`..` will expand to as many values as necessary.

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }
}
```

However, using .. must be unambiguous. If it is unclear which values are intended for matching and which should be ignored,
Rust will give us an error.
The following code does not compile:

```rust
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}
```

## Extra Conditionals with Match Guards

A _mathch guard_ is an additional `if` condition specified after a `match` arm's pattern that must also match along
with the matching pattern for that arm to be chosen.
The condition can use variables created in the pattern.

```rust
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // Matches for 4, but not for 10
        Some(x) => println!("{}", x),
        None => (),
    }
}
```

There is no way to express the `if x < 5` condition within a pattern, so the match guard gives us the ability to express this logic.

A use case for match guards is to use outer scope values without shadowing them.

```rust
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
```

The match guard if n == y is not a pattern and therefore doesn’t introduce new variables.
This y is the outer y rather than a new shadowed y, and we can look for a value that has the
same value as the outer y by comparing n to y.

You can also use the or operator `|` in a match guard to specify multiple patterns;
the match guard condition will apply to all the patterns.

```rust
fn main() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // This is like (4 | 5 | 6) if y
        _ => println!("no"),
    }
}
```

## @ Bindings

The `@` operator lets us create a variable that holds a value at the same time we're testing that value to see
if it matches a pattern.

```rust
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // we want to test that a Message::Hello id field is within the range 3..=7.
        // But we also want to bind the value to the variable id_variable so we can use it in the code associated with the arm.
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```

By specifying `id_variable @` before the range `3..=7`, we’re capturing whatever value matched the range while also
testing that the value matched the range pattern.

Using `@` lets us test a value and save it in a variable within one pattern.
