#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // Extracts value from the enum variant
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// Matches in Rust are exhaustive: we must exhaust every last possibility in
// order for the code to be valid. Especially in the case of Option<T>, when
// Rust prevents us from forgetting to explicitly handle the None case, it
// protects us from assuming that we have a value when we might have null

// Wildcard cases (or, say, default cases) are represented using _
fn match_u8_value(x: u8) {
    match x {
        1 => println!("one"),
        3 => println!("three"),
        _ => println!("not one or three"),
    }
}

fn main() {
    let quarter = Coin::Quarter(UsState::Alaska);

    println!("Value in cents: {}", value_in_cents(quarter));
    // "State quarter from Alaska!"

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    match_u8_value(2);
    match_u8_value(3);
}

// However, the match expression can be a bit wordy in a situation in which we
// care about only one of the cases. For this situation, Rust provides if let.
