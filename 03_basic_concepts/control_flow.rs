fn main() {
    let number = 3;

    if number < 5 { // Condition MUST be a bool. No automatic type conversions :)
        println!("true");
    } else { // `else if`
        println!("false");
    }

    // if in an expression
    let new_number = if number < 5 { number } else { number + 5 };
    // Note that types of both branches here CANNOT be different

    // loop
    // Infinite loop without conditions
    // Used for tasks such as waiting for some thread
    // Example:
    // loop {
    //     println!("Infinite");
    // }
    // Can return a value via `break`
    let mut counter = 1;
    let counter_max = loop {
        counter += 1;

        if counter == 10 {
            break counter; // Returns a value
        }
    };

    // while
    while counter < 100 {
        counter *= 2;
    }

    // for
    let a = [10, 20, 30, 40];
    for element in a.iter() {
        println!("{}", element);
    }

    // Using ranges with for
    for num in (1..11).rev() {
        println!("{}", num);
    }
}