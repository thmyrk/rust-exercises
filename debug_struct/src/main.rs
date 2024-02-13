#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 2,
        height: 3,
    };

    println!("Width: {0}, Height: {1}", rectangle.width, rectangle.height);
    dbg!(&rectangle);
}
