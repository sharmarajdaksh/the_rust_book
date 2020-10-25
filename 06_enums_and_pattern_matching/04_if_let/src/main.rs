#[derive(Debug)]
enum State {
    Alaska,
    Alabama,
}

enum Coin {
    Penny,
    Quarter(State),
}

fn main() {
    println!("Hello, world!");

    // The if let syntax lets you combine if and let into a less verbose way
    // to handle values that match one pattern while ignoring the rest.

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // Do nothing
    }

    // Same as
    if let Some(3) = some_u8_value {
        // if let takes a pattern and an expression separated by an equal sign
        println!("three");
    }

    // if let can also have an else block
    let coin = Coin::Quarter(State::Alabama);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    // Same as
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
