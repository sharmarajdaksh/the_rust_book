// When we use a parameter in the body of the function, we have to declare the
// parameter name in the signature so the compiler knows what that name means.
// Similarly, when we use a type parameter name in a function signature, we
// have to declare the type parameter name before we use it.

// Generic function
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = list[0];

//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     // Using generic function for i32
//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     // Using generic function for char
//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }
//
// the body of largest won’t work for all possible types that T could be.
// Because we want to compare values of type T in the body, we can only use
// types whose values can be ordered. To enable comparisons, the standard
// library has the std::cmp::PartialOrd trait that you can implement on types
//

struct PointXY<T, U> {
    x: T,
    y: U,
}

// Generic type parameters in a struct definition aren’t always the same as
// those you use in that struct’s method signatures.
impl<T, U> PointXY<T, U> {
    fn mixup<V, W>(self, other: PointXY<V, W>) -> PointXY<T, W> {
        PointXY {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

// Generics in type implementations/methods
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We could, for example, implement methods only on Point<f32> instances rather
// than on Point<T> instances with any generic type.
impl Point<f32> {
    // This code means the type Point<f32> will have a method named
    // distance_from_origin and other instances of Point<T> where T is not of
    // type f32 will not have this method defined.
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let both_integer = PointXY { x: 5, y: 10 };
    let both_float = PointXY { x: 1.0, y: 4.0 };
    let integer_and_float = PointXY { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = PointXY { x: 5, y: 10.4 };
    let p2 = PointXY { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

//
// As we did with structs, we can define enums to hold generic data types in
// their variants.
// enum Option<T> {
//     Some(T),
//     None,
// }
//
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
//

//
// Rust implements generics in such a way that your code doesn’t run any slower
// using generic types than it would with concrete types.

// Rust accomplishes this by performing monomorphization of the code that is
// using generics at compile time. Monomorphization is the process of turning
// generic code into specific code by filling in the concrete types that are
// used when compiled.
