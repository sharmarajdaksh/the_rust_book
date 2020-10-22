// Define a struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// In the User struct definition, we used the owned String type rather than the &str string slice type.
// This is a deliberate choice because we want instances of this struct to own all of its data and for that data to be valid for as long as the entire struct is valid.
// It’s possible for structs to store references to data owned by something else, but to do so requires the use of Rust `lifetimes`.
// Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
// So, essentially, without lifetimes, we couldn't have used &str instead of String in User

fn build_user(email: String, username: String) -> User {
    User {
        email, // Similar to Javascript
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        // Must be made mutable explicitly to allow modifications
        // Rust does not allow you to make only certain fields mutable
        username: String::from("someone123"),
        email: String::from("someone123@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("someone@example.com");

    let user2 = build_user(
        String::from("newuser2@example.com"),
        String::from("newuser2"),
    );

    // Javascript-like "struct update syntax"
    let user3 = User {
        email: String::from("newuser3@example.com"),
        username: String::from("newuser3"),
        ..user2 // Unpack user2 to fill in remaining fields
    };

    // Tuple structs are a thing
    struct Color(i32, i32, i32); // A new type
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // if black and origin were tuples, then black == origin would return true
    // Not so now. They are both different types.
    let black_red = black.0; // Accessible just like tuples

    // You can also define structs that don’t have any fields
    // These are called unit-like structs because they behave similarly to (), the unit type.
    // Unit-like structs can be useful in situations in which you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    // Basically like a Go struct without data
}
