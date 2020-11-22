// Implementing the `Drop` trait, you can control what happens when
// a value is about to go out of scope
// Used mainly alongside smart pointer implementations

// Variables get dropped in reverse the order of creation

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // It is also possible to drop a value early
    // While there is no way to manually invoke the drop function
    // in the Drop trait implementation,
    // This can be done using std::mem::drop
    //
    let e = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("e created");

    std::mem::drop(e);
    println!("e dropped before end of scope");
}
