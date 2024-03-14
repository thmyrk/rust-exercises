struct Point<T, U> {
    x: T,
    y: U,
}

impl<T: std::marker::Copy, U: std::marker::Copy> Point<T, U> {
    fn mixup<X: std::marker::Copy, Y: std::marker::Copy>(
        &self,
        point: &Point<X, Y>,
    ) -> Point<T, Y> {
        Point {
            x: self.x,
            y: point.y,
        }
    }
}

fn main() {
    let point = Point { x: 0, y: 'a' };
    let other_point = Point { x: 1.1, y: "hello" };
    let new_point = point.mixup(&other_point);

    println!("New point {0} {1}", new_point.x, new_point.y);
    println!("Point {0} {1}", point.x, point.y);
    println!("Other point {0} {1}", other_point.x, other_point.y);
}
