use itertools::Itertools;

// https://www.codewars.com/kata/5263c6999e0f40dee200059d/train/rust

fn get_pins(observed: &str) -> Vec<String> {
    let neighbours: Vec<Vec<&str>> = vec![
        vec!["0", "8"],
        vec!["1", "2", "4"],
        vec!["1", "2", "3", "5"],
        vec!["2", "3", "6"],
        vec!["1", "4", "5", "7"],
        vec!["2", "4", "5", "6", "8"],
        vec!["3", "5", "6", "9"],
        vec!["4", "7", "8"],
        vec!["0", "5", "7", "8", "9"],
        vec!["6", "8", "9"],
    ];

    observed
        .chars()
        .map(|pin_char| neighbours.get(char_to_digit(pin_char)).unwrap())
        .multi_cartesian_product()
        .map(|vec| vec.iter().map(|s| *s).join(""))
        .collect::<Vec<String>>()
}

fn char_to_digit(c: char) -> usize {
    usize::try_from(c.to_digit(10).unwrap()).unwrap()
}

fn main() {
    println!("For observed PIN: 8");
    println!("the possible results are: {:?}", get_pins("8"));
    println!("For observed PIN: 11");
    println!("the possible results are: {:?}", get_pins("11"));
    println!("For observed PIN: 369");
    println!("the possible results are: {:?}", get_pins("369"));
}
