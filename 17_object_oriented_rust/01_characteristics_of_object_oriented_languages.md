# Charactetistics of Object Oriented Languages

Objects contain Data and Behavior

- Structs and enums have data, and `impl` blocks provide methods on them
- Even though these aren't called objects. they provide the same functionality

Encapsulation to hide implementation details

- Enabled using the `pub` keyword to only expose required modules, types, functions

Inheritance as a type system and as code sharing

- Rust has no support for inheritance
- Alternatively, behavior can be reused by using traits and providing default trait implementations. This default implementation can be overriden in implementing types.
- To enable polymorhism, which is another use case for inheritance, can be implemented via generics (this is called bounded parametric polymorphism).
