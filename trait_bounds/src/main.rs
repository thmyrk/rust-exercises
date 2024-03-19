trait Summary {
    fn summarize(&self) -> String;
}

trait Capitalize {
    fn capitalize(&self) -> String;
}

impl<T: Summary> Capitalize for T {
    fn capitalize(&self) -> String {
        self.summarize().to_uppercase()
    }
}

enum NumericTypes {
    Integer(i64),
    Float(f64),
}

impl std::fmt::Display for NumericTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericTypes::Integer(num) => write!(f, "{}", num),
            NumericTypes::Float(num) => write!(f, "{}", num),
        }
    }
}

struct Point {
    x: NumericTypes,
    y: NumericTypes,
}

impl Summary for Point {
    fn summarize(&self) -> String {
        format!("Point summary is X: {0}, Y: {1}", self.x, self.y)
    }
}

fn describe_item<T: Summary>(item: &T) -> String {
    format!(
        "{0}. Didn't you listen? {1}",
        item.summarize(),
        item.capitalize()
    )
}

fn main() {
    let point = Point {
        x: NumericTypes::Integer(3),
        y: NumericTypes::Float(4.2),
    };

    println!("Point coordinates are {0} {1}", point.x, point.y);
    println!("{}", describe_item(&point));
}
