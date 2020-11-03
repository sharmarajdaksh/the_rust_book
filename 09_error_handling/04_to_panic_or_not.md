Generally, by default, it may be preferable to return a `Result<T,E>` type rather
than calling `panic!`.

In tests, a failed test should call `panic!` to indicate failure.

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state.
In this context, a bad state is when some assumption, guarantee, contract, or invariant has been
broken, such as when invalid values, contradictory values, or missing values are passed to your code
—plus one or more of the following:
- The bad state is not something that’s expected to happen occasionally.
- Your code after this point needs to rely on not being in this bad state.
- There’s not a good way to encode this information in the types you use.

If someone calls your code and passes in values that don’t make sense, 
the best choice might be to call panic! and alert the person using your 
library to the bug in their code so they can fix it during development. 
Similarly, panic! is often appropriate if you’re calling external code that 
is out of your control and it returns an invalid state that you have no way of fixing.

However, when failure is expected, it’s more appropriate to return a 
Result than to make a panic! call. Examples include a parser being given 
malformed data or an HTTP request returning a status that indicates you 
have hit a rate limit. In these cases, returning a Result indicates that 
failure is an expected possibility that the calling code must decide how to handle.

Sometimes, if validations were extremely valid to code, we could create custom
types with implemented validations in order to ensure that those conditions
are always satisfied. That way, it’s safe for functions to use the new type 
in their signatures and confidently use the values they receive.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    // Factory pattern
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // Getter is necessary since value is private
    pub fn value(&self) -> i32 {
        self.value
    }
}
```


