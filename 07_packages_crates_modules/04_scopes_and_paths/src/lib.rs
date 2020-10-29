mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Bring module into scope
// No longer need to specify full paths
use crate::front_of_house::hosting;
// use can also be used with relative paths:
// use self::front_of_house::hosting;

// you could also have imported add_to_waitlist itself
// use self::front_of_house::hosting::add_to_waitlist;
// However, usage as currently done is the idiomatic way for functions
// It makes clear that the function isn't locally defined

// For structs, enums, and other items, it is idiomatic
// to specify the full path, such as:
//
// use std::collections::HashMap;
//
// fn main() {
//     let mut map = HashMap::new();
//     map.insert(1, 2);
// }

// as is usable
//
// use std::fmt::Result;
// use std::io::Result as IoResult;
//
// fn function1() -> Result {}
// fn function2() -> IoResult<()> {}

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// When you bring a name into scope with `use`
// It is private by default, but can be made public using `pub`
// This techique is called re-exporting
pub use crate::front_of_house::hosting::add_to_waitlist;

// Netsted import paths are possible
//
// These two are equivalent
//
use std::cmp::Ordering;
use std::io;
// SAME AS
use std::{cmp::Ordering, io};
//
// Similarly, you can also replace this:
use std::io;
use std::io::Write;
// with using `self`
use std::io::{self, Write};

// * wildcard is also available with use
// Called the 'Glob' operator
use std::collections::*;
