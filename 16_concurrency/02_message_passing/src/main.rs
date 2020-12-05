// The idea is to "not communicate by sharing memory; instead
// share memory by communicating"
// Rust provides an implementation of channels for message passing
//

use std::sync::mpsc;
// mpsc - multiple producers, single consumer
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();
    // tx is the sending end of the channel (transmitter)
    // tx is the receiving end of the channel (receiver)

    let tx2 = mpsc::Sender::clone(&tx1);
    // Clone the first transmitter to allow for multiple transmitters

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val) // Returns a Result<T,E>
                .unwrap(); // Panics on Error (channel closed)
                           // The ownership of val is now lost.
                           // Following is invalid:
                           // println!("Val is {}", val)
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx2.send(val) // Returns a Result<T,E>
                .unwrap(); // Panics on Error (channel closed)
                           // The ownership of val is now lost.
                           // Following is invalid:
                           // println!("Val is {}", val)
            thread::sleep(Duration::from_secs(1));
        }
    });

    // recv blocks the main thread
    // try_recv doesn't block. Instead, it returns a Result<T,E> immediately
    // Ok if a message is available or Err otherwise.
    // try_recv is useful if  athread had to do other work while waiting,
    // say, in a loop that calls try_recv regularly
    for received in rx {
        println!("Got: {}", received);
    }
}
