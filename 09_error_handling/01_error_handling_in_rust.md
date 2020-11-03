Most languages don’t distinguish between these two kinds of errors and handle 
both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. 
Instead, it has the type Result<T, E> for recoverable errors and the panic! macro 
that stops execution when the program encounters an unrecoverable error.
