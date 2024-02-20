// create a vector
// add enum types to vector
// remove something from vector

#[derive(Debug)]
enum Things {
    First(i32),
    Second(String),
    Third(bool),
}

fn print_vector(vector: &Vec<Things>) {
    for element in vector {
        println!("Here's a vector element {:?}", element);
    }

    if let Some(value) = vector.get(0) {
        println!("Get(0) is {:?}", value);
    }

    match vector.get(3) {
        Some(value) => println!("Get(3) is {:?}", value),
        None => println!("Get(3) has no value!"),
    }
}

fn add_new_element(vector: &mut Vec<Things>) {
    vector.push(Things::First(100));
    vector.push(Things::Third(false));
}

fn main() {
    let mut vector = vec![
        Things::First(123),
        Things::Second(String::from("Hello")),
        Things::Third(true),
    ];

    // let element = &vector[0];

    print_vector(&vector);
    add_new_element(&mut vector);
    print_vector(&vector);

    // println!("{:?}", element);
    // the lines are commented because of an immutable borrow being used after a mutable one
}
