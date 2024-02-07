fn main() {
    let loop_result = 'outer_loop: loop {
        println!("Outer loop");
        loop {
            println!("Inner loop");
            break 'outer_loop 5;
        }
    };

    println!("Loop result: {loop_result}")
}
