fn main() {
    let s1 = String::from("hello");
    let (s2, s3) = take_and_return(s1);
    // s1 is invalid at this point
    // println!("main {s1}") will throw an error

    println!("main {s2}");
    println!("main {s3}");
}

fn take_and_return(mut input_string: String) -> (String, String) {
    println!("take_and_return {input_string}");

    input_string.push_str(" world");
    
    return (input_string, String::from("new string"))
}
