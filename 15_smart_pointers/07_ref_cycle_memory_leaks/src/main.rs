// Preventing reference cycles by turning Rc<T> into Weak<T>
//
// Rc::clone increases the strong_count of an Rc<T>. An Rc<T> is cleaned
// up only if strong_count==0
// You can also create a weak reference to the value within an Rc<T>
// instance by calling _Rc::downgrade_ and passing a reference to the Rc<T>,
// which returns a `Weak<T>` instead of incrementing strong_count
// Rc<T> tracks weak_count as well.
// The difference is that weak_count does not need to be 0 for the
// Rc<T> to be cleaned up
//
// Strong references are how you can share ownership of an Rc<T> instance.
// Weak references don’t express an ownership relationship.
// They won’t cause a reference cycle because any cycle involving some weak
// references will be broken once the strong reference count of values
// involved is 0.
//
// Because the value that Weak<T> references might have been dropped,
// to do anything with the value that a Weak<T> is pointing to, you must
// make sure the value still exists.
// Do this by calling the upgrade method on a Weak<T> instance, which will
// return an Option<Rc<T>>. You’ll get a result of Some if the Rc<T> value
// has not been dropped yet and a result of None if the Rc<T> value has
// been dropped.
//

use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // We want a Node to own its children, and we want to share that ownership
    // with variables so we can access each Node in the tree directly.
    // To do this, we define the Vec<T> items to be values of type Rc<Node>.
    // We also want to modify which nodes are children of another node, so we
    // have a RefCell<T> in children around the Vec<Rc<Node>>.
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch), // 1
            Rc::weak_count(&branch),   // 1
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // 2
            Rc::weak_count(&leaf),   // 0
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // None
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf), // 1
        Rc::weak_count(&leaf),   // 0
    );
}
