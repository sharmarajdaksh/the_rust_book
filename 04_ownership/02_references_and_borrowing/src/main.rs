fn main() {
    let s1 = String::from("hello");
    let l = calculate_length(&s1); // Ownership is not lost

    println!("The length of {} is {}", s1, l);

    let mut s2 = String::from("Hello");
    change(&mut s2); // Pass reference as mutable
                     // Note that s2 also has to be mutable for this to work

    // But mutable references have one big restriction: you can have only one
    // mutable reference to a particular piece of data in a particular scope. /
    // This code will fail:
    //      let r1 = &mut s2;
    //      let r2 = &mut s2; // r2 will cause compile failure:
    //                  // Cannot borrow 's2' as mutable more than once at a time

    //      println!("{} {}", r1, r2);
    // This rule also exists when combining mutable and immutable references
    // - We cannot have a mutable reference while we have an immutable one.
    // - However, multiple immutable references are fine

    // The benefit is that it prevents data races at compile time. They occur in situations where:
    // - Two or more pointers access the same data at the same time
    // - At least one of the pointers is writing
    // - There is no mechanism being used to sync data accesss

    // Note that a reference’s scope starts from where it is introduced and
    // continues through the last time that reference is used. For instance,
    // this code will compile because the last usage of the immutable
    // references occurs before the mutable reference is introduced:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // On dangling pointers:
    // Code will simply not compile if there is the potential for some dangling pointers
}

fn calculate_length(s: &String) -> usize {
    // Passing references as function parameters is called "borrowing"
    println!("The passed reference points to the string is \"{}\"", *s);
    s.len()
} // s goes out of scope here
  // But since it did not have ownership of the data it refers to, drop() does not need to be called

// Note that references are mutable by default, like variables
// The following function is hence invalid:
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
