// Implements the std::fmt::Debug trait for the struct,
// allowing the {:?} (single line formatted) or {:#?} (better,
// indented formatting) in the println! to work
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are functions defined in the context of a struct
// The first parameter is always `self`
impl Rectangle {
    // Implementation (impl) block for struct Rectangle

    // A method, called as obj.area()
    fn area(&self) -> u32 {
        // The & is simply added to avoid loss of ownership
        self.width * self.height
    }

    // A method, called as obj.can_hold(obj2)
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    // An associated method, than can be called as
    // Rectangle::square(n)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }

    // Technically you could have multiple impl blocks if you wanted
    // No specific reason to do it though
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:#?}", rect1);
    println!("area of rect1 is {}", rect1.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = Rectangle::square(3);
    println!("rect4 is {:#?}", rect4);
    println!("area of rect4 is {}", rect4.area());
}

// Generic implementation that is not closely bound to the struct
// println!("area of rect1 is {}", area(&rect1));
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
