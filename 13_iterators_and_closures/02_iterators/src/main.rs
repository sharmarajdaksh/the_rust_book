// Rust iterators are lazy: they have no effect until you call
// methods that consume the iterator to use it up
fn main() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    // Calling next() mutates internal state, hence requires mutability
    // However, using it in a loop does not require this explicit declaration
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // The values we get from the calls to next are immutable references to the
    // values in the vector. The iter method produces an iterator over
    // immutable references. If we want to create an iterator that takes
    // ownership of v1 and returns owned values, we can call into_iter instead
    // of iter. Similarly, if we want to iterate over mutable references, we
    // can call iter_mut instead of iter.
    for val in v1.iter() {
        println!("Got: {}", val);
    }

    // Methods defined on the Iterator trait, known as iterator adaptors, allow
    // you to change iterators into different kinds of iterators. You can chain
    // multiple calls to iterator adaptors to perform complex actions in a
    // readable way. But because all iterators are lazy, you have to call one
    // of the consuming adaptor methods to get results from calls to iterator
    // adaptors.
    let v1 = vec![1, 2, 3];
    // v1.iter().map(|x| x + 1); // This iterator must be consumed because it
    //                           // is useless otherwise
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// takes ownership of shoes vector
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// All iterators implement the Iterator trait
// // pub trait Iterator {
//     type Item; // Associated type
//
//     fn next(&mut self) -> Option<Self::Item>;
//
//     // methods with default implementations elided
// }

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// Implementing the Iterator trait
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Methods that call next are called consuming adaptors, because calling
    // them uses up the iterator. One example is the sum method, which takes
    // ownership of the iterator and iterates through the items by repeatedly
    // calling next, thus consuming the iterator. As it iterates through, it
    // adds each item to a running total and returns the total when iteration
    // is complete.
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
        // Can't use v1_iter after call to sum because sum takes ownership of the
        // iterator we call it on
        assert_eq!(total, 6);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
