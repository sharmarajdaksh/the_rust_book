// Use case for Interior MutabilityL Mock Objects

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// When creating immutable and mutable references, we use the & and &mut
// syntax, respectively. With RefCell<T>, we use the borrow and borrow_mut
// methods, which are part of the safe API that belongs to RefCell<T>.
// The borrow method returns the smart pointer type Ref<T>, and borrow_mut
// returns the smart pointer type RefMut<T>.
// Both types implement Deref, so we can treat them like regular references.

// The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart
// pointers are currently active. Every time we call borrow,
// the RefCell<T> increases its count of how many immutable borrows
// are active. When a Ref<T> value goes out of scope, the count of
// immutable borrows goes down by one. Just like the compile-time borrowing
// rules, RefCell<T> lets us have many immutable borrows or one mutable
// borrow at any point in time.

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // sent_messages: Vec<String>,
        sent_messages: RefCell<Vec<String>>,
    }

    // Mock object to test a Messenger which, if this code is a lib,
    // will be implemented by the user to Send however they want
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // sent_messages: vec![],
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // This errors
            // self.sent_messages.push(String::from(message));
            // We can’t modify the MockMessenger to keep track of the messages,
            // because the send method takes an immutable reference to self.
            // We also can’t take the suggestion from the error text to use
            // &mut self instead, because then the signature of send wouldn’t
            // match the signature in the Messenger trait definition
            //
            // Refcell to the rescur
            self.sent_messages
                .borrow_mut() // borrow a mutable reference
                .push(String::from(message));

            // Following code compiles, but causes panic! at tuntime.
            // REASON: It attempts to create two mutable borrows active for the
            // same scope
            // let mut one_borrow = self.sent_messages.borrow_mut();
            // let mut two_borrow = self.sent_messages.borrow_mut();

            // one_borrow.push(String::from(message));
            // two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // assert_eq!(mock_messenger.sent_messages.len(), 1);
        assert_eq!(
            mock_messenger
                .sent_messages
                .borrow() // to see how many items are in the inner vector, we
                // call borrow on the RefCell<Vec<String>>
                // to get an immutable reference to the vector.
                .len(),
            1
        );
    }
}
