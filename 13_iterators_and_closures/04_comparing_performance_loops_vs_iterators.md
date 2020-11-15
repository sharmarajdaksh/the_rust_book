Iterators, although a high-level abstraction, get compiled down to roughly the
same code as if you’d written the lower-level code yourself. Iterators are one
of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes
no additional runtime overhead. This is analogous to how Bjarne Stroustrup, the
original designer and implementor of C++, defines zero-overehad in "Foundations of C++":

> In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better.
