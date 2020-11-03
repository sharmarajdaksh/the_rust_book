fn main() {
    // Vectors are implemented using generic
    // Specifying type (explicitly or initializing) is thus necessary
    let v: Vec<i32> = Vec::new();

    // Vectors can also be initialized using the macro
    let mut v = vec![1, 2, 3]; // Rust infers i32

    v.push(5); // v must be mutable for this to be possible
    v.push(6);
    v.push(7);

    // Like structs, a vector is freed when it goes out of scope
    // Accessing vector values using &[]
    let third = &v[2]; // Will cause panic on overflow access
    println!("The third element is {}", third);
    v.push(5);

    // Accessing vector values using get()
    match v.get(2) {
        // returns an Option<T>
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    // Unlike &[], get does not cause panic on invalid index access
    // It simply returns None

    // Here as well, you cannot have both mutable and immutable references
    // in the same scope
    //
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6); // Raises error
    // println!("The first element is: {}", first);

    // Iteration
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // Dereferencing to modify value
    }

    // Vectors of enum type are also valid
    // Using enum variants we can thus have items of different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
