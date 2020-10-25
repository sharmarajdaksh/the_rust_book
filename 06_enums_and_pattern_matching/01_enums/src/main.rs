// Basic enum
// enum IpAddrKind {
//     V4,
//     V6,
// }

// enums can also have associated data????
// with each variant having different data
enum IpAddr {
    // Might often be advantageous over structs to hold
    // Varying but related data types
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Similar to structs
impl Message {
    fn call(&self) {
        // do something
        println!("{:#?}", &self)
    }
}

// Using enums as types
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main() {
    // For the basic enum case

    // Accessing enum fields
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    // For the associated data enum

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();
}

// You could have functions with enum type as parameter, that should support
// all enum variants
// fn route(ip_kind: IpAddrKind) {
//     // do something
// }
