# Refutability: Whether a pattern might fail to match

Patterns may be refutable or irrefutable. Irrefutable patterns are those that would match for any possible values.
`let x = 5;` is an irrefutable pattern, while something like `if let Some(x) = a_value` is refutable, because if the value
in `a_value` is `None` rather than `Some`, the `Some(x)` pattern would not match.

Functional parameters, `let` statements, and `for` loops can only accept irrefutable patterns.
`if let` and `while let` accept refutable as well as irrefutable patterns, but the compiler warns against
irrefutable pattens because by definition they're supposed to handle possible failure.

This, for example, is invalid code:

```rust
let Some(x) = some_option_value;
```

If `some_option_value` was None, it would fail to match the pattern. However, the `let` statement can only accept
irrefutable patterns.

This, however, would be valid:

```rust
if let Some(x) = some_option_value {
    println!("{}", x);
}
```

The following code would give a compiler warning, since the pattern would always match:

```rust
if let x = 5 {
    println("{}", x);
}
```

For this reason, match arms must use refutable patterns, except for the last arm, which should match any
remaining values with an irrefutable pattern.
Rust allows us to use an irrefutable pattern in a match with only one arm, but this syntax isn’t particularly useful and could
be replaced with a simpler let statement.
