// Rust has only one string type in the core language: the string slice str
// String slices are references to some UTF-8 encoded string data

// The String type (collection) is growable, mutable, owned, UTF-8 encoded

// Both string slices (str or &str) and String are widely used

// Rust’s standard library also includes a number of other string types,
// such as OsString, OsStr, CString, and CStr

fn main() {
    let mut s = String::new();
    let data = "initial contents";
    s = data.to_string();

    // Same as
    let s = String::from(data);

    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    s.push_str(" appended"); // appended
                             // takes a string slice as argument so as to not
                             // transfer ownership

    s.push('z'); // appends single character

    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2; // s1 is now moved, and no longer valid
                       // We can only add a &str to a String;
                       // We can't add two String values together
                       // The compiler coerces the &String to a &str
                       // &s2 becomes &s2[..]

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The format! macro
    let s = format!("{}-{}-{}", s1, s2, s3);
    // Same as
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // Unlike add (+), format! does not take away ownership of s1

    // Rust String data type does not allow string indexing
    // let hello = "Здравствуйте";
    // let answer = &hello[0]; // This is invalid
    // Slicing, however, is invalid
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Зд (which is 2 characters but 4 bytes)
                          // However, using [0..1] would cause panic :\

    // Still, you can iterate over utf-8 characters ina  string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
