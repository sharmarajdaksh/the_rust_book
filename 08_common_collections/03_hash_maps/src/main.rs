use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Using zip and collect to form a HashMap
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    // HashMap<_, _> is necessary because collect can collect into many data types

    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be
    // moved and the hash map will be the owner of those values
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point

    // Accessing HashMap values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Overwriting a value if it exists
    scores.insert(String::from("Blue"), 25);

    // Insert only if key has no value
    //
    // The return value of the entry method is an enum called Entry that
    // represents a value that might or might not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    // The or_insert method on Entry is defined to return a mutable reference
    // to the value for the corresponding Entry key if that key exists, and if
    // not, inserts the parameter as the new value for this key and returns a
    // mutable reference to the new value.

    // Updating a value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        // The or_insert method actually returns a mutable reference (&mut V)
        // to the value for this key. Here we store that mutable reference in
        // the count variable, so in order to assign to that value, we must
        // first dereference count using the asterisk (*). The mutable
        // reference goes out of scope at the end of the for loop, so all of
        // these changes are safe and allowed by the borrowing rules.
    }

    println!("{:?}", map);
}
