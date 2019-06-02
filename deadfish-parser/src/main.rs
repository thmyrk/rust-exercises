// Write a simple parser that will parse and run Deadfish.
// Deadfish has 4 commands, each 1 character long:
// i increments the value (initially 0)
// d decrements the value
// s squares the value
// o outputs the value into the return array

fn parse(code: &str) -> Vec<i32> {
    let mut value = 0;
    let mut result = Vec::new();

    for character in code.chars() {
        match character {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value = value * value,
            'o' => result.push(value),
            _ => {},
        }
    }
    result
}

#[test]
fn run_tests() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
