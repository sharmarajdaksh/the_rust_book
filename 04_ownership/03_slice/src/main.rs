fn main() {
    // Another data type that does not have ownership is the slice.
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.

    let mut s = String::from("hello world");
    let hello = &s[0..5]; // can also drop the zero to use [..5]
    let world = &s[6..11]; // can also drop the 11 to use [6..]

    let mut s = String::from("");

    let word = first_word(&s);

    s.clear(); // Errors since it needs a mutable ref
               // Which cannot be acquired due to there already being an immutable reference

    println!("The first word is {}", word);

    // String literals are slices
    let s = "Hello, world!";
    // The type of s is &str
    // It is a slice pointeing to that specific point of the binary
    // String literals are immutable for this reason

    // Like string slices, you can also have array slices

    // Slices work by storing a reference to the first element and a length
}

fn first_word(s: &str) -> &str {
    // s may be String or str reference
    let bytes = s.as_bytes(); // Convert to bytes

    for (i, &item) in bytes.iter().enumerate() {
        // Iterate
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..] // if no space is found, whole String is a word
}
