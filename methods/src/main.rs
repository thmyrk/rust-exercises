struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new_square(width: u32) -> Rectangle {
        Self {
            width: width,
            height: width,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width >= rectangle.width && self.height >= rectangle.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 32,
        height: 32,
    };

    let rect2 = Rectangle {
        width: 32,
        height: 33,
    };

    let rect3 = Rectangle {
        width: 4,
        height: 6,
    };

    println!("Area of rectangle is: {0}", rect1.area());
    println!("rect1 can hold rect2: {0}, rect1 can hold rect3: {1}", rect1.can_hold(&rect2), rect1.can_hold(&rect3));

    let square = Rectangle::new_square(5);
    println!("Area of a square with a width of 5 is: {}", square.area());
    ()
}
