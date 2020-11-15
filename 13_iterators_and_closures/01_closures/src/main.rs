use std::thread;
use std::time::Duration;

// Closures are anonymous functions you can save in variables
// or pass as arguments to functions. Unlike functions, these
// can capture values from the scope in which they are defined

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

//
// BASIC CLOSURE-BASED APPROACH
//
// fn generate_workout(intensity: u32, random_number: u32) {
//     // Store expensive code in a variable as a closure
//     // Rather than calling the function and saving the result
//     let expensive_closure = |num| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         num
//     };
//     // The variable contains the definition of an anonymous function
//     // rather than the resulting value

//     // Type annotations are optional
//     // let expensive_closure = |num: u32| -> u32 {
//     //     println!("calculating slowly...");
//     //     thread::sleep(Duration::from_secs(2));
//     //     num
//     // };

//     // Unlike using
//     // let expensive_closure = simulated_expensive_calculation(intensity);
//     // Using a closure means that the fuction isn't called (and made to
//     // take up 2 seconds) as soon as generate_workout is called. Instead,
//     // the expensive calculation happens only when actually needed

//     // DRAWBACK
//     // Even using closures, the closure is evaluated twice in the case
//     // That intensity is less than 25
//     // BETTER APPROACH:
//     // Implementing some kind of memoization

//     if intensity < 25 {
//         println!("Today, do {} pushups!", expensive_closure(intensity));
//         println!("Next, do {} situps!", expensive_closure(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", expensive_closure(intensity));
//         }
//     }
// }

//
// MEMOIZATION-BASED APPROACH
//
struct Cacher<T>
where
    T: Fn(u32) -> u32,
    // The Fn traits are provided by the standard library. All closures
    // implement at least one of the traits: Fn, FnMut, or FnOnce.
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // The result gets cached on the first call
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Closures can also capture value from the environment
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));
}

// Closure definitions will have one concrete type inferred for each of their
// parameters and for their return value. Something like this is thus invalid:
//
// let example_closure = |x| x;
//
// let s = example_closure(String::from("hello"));
//                  // The closure's x is now inferred to be of type string
// let n = example_closure(5);

// Closures can capture values from their environment in three ways, which
// directly map to the three ways a function can take a parameter: taking
// ownership, borrowing mutably, and borrowing immutably. These are encoded in
// the three Fn traits as follows:
// - FnOnce consumes the variables it captures from its enclosing scope, known
//   as the closure’s environment. To consume the captured variables, the
//   closure must take ownership of these variables and move them into the
//   closure when it is defined. The Once part of the name represents the fact
//   that the closure can’t take ownership of the same variables more than
//   once, so it can be called only once.
// - FnMut can change the environment because it mutably borrows values.
// - Fn borrows values from the environment immutably.
//
// When you create a closure, Rust infers which trait to use based on how the
// closure uses the values from the environment. All closures implement FnOnce
// because they can all be called at least once. Closures that don’t move the
// captured variables also implement FnMut, and closures that don’t need
// mutable access to the captured variables also implement Fn

// If you want to force the closure to take ownership of the values it uses in
// the environment, you can use the move keyword before the parameter list.
// This technique is mostly useful when passing a closure to a new thread to
// move the data so it’s owned by the new thread.
// Following code doesn't compile since the closure takes ownership
// fn main() {
//     let x = vec![1, 2, 3];

//     let equal_to_x = move |z| z == x;

//     println!("can't use x here: {:?}", x);

//     let y = vec![1, 2, 3];

//     assert!(equal_to_x(y));
// }
