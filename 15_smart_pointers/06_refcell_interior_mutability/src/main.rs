// Interior mutability in Rust is a design pattern, allowing
// you to mutate immutable references to the data.
// Normally, this is disallowed by borrowing rules. It uses `unsafe` code
// inside a data structure to bend Rust's usual rules
// We can use types that use the interior mutability pattern when we can
// ensure that the borrowing rules will be followed at runtime, even
// though the compiler can’t guarantee that. The unsafe code involved is
// then wrapped in a safe API, and the outer type is still immutable.

// With references and Box<T>, the borrowing rules’ invariants are enforced
// at compile time. With RefCell<T>, these invariants are enforced at runtime
// With references, if you break these rules, you’ll get a compiler error.
// With RefCell<T>, if you break these rules, your program will panic and exit.

// The advantage of checking the borrowing rules at runtime instead is
// that certain memory-safe scenarios are then allowed, whereas they are
// disallowed by the compile-time checks.
// Static analysis, like the Rust compiler, is inherently conservative.
// Some properties of code are impossible to detect by analyzing the code:
// the most famous example is the Halting Problem

// Similar to Rc<T>, RefCell<T> is only for use in single-threaded
// scenarios and will give you a compile-time error if you try using it
// in a multithreaded context.

// A consequence of the borrowing rules is that when you have an immutable
// value, you can’t borrow it mutably
// This code is not valid:
// let x = 5;
// let y = &mut x;
// However, there are situations in which it would be useful for a value
// to mutate itself in its methods but appear immutable to other code.
// Code outside the value’s methods would not be able to mutate the
// value. Using RefCell<T> is one way to get the ability to have
// interior mutability. But RefCell<T> doesn’t get around the borrowing
// rules completely: the borrow checker in the compiler allows this
// interior mutability, and the borrowing rules are checked at runtime
// instead. If you violate the rules, you’ll get a panic!

// Multiple owners of Mutable data by combining Rc<T> and RefCell<T>
//
// A common way to use RefCell<T> is in combination with Rc<T>. Recall that ]
// Rc<T> lets you have multiple owners of some data, but it only gives
// immutable access to that data. If you have an Rc<T> that holds a
// RefCell<T>, you can get a value that can have multiple owners and
// that you can mutate!

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
    // OUTPUT
    // a after = Cons(RefCell { value: 15 }, Nil)
    // b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    // c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}

// This technique is pretty neat! By using RefCell<T>, we have an
// outwardly immutable List value. But we can use the methods on
// RefCell<T> that provide access to its interior mutability so we can
// modify our data when we need to.
// The runtime checks of the borrowing rules protect us from data races,
// and it’s sometimes worth trading a bit of speed for this flexibility
// in our data structures.

// The standard library has other types that provide interior mutability,
// such as Cell<T>, which is similar except that instead of giving
// references to the inner value, the value is copied in and out of the
// Cell<T>. There’s also Mutex<T>, which offers interior mutability that’s
// safe to use across threads;
