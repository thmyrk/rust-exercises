// https://www.codewars.com/kata/duplicate-encoder/train/rust
// The goal of this exercise is to convert a string to a new string where each character in the new string is "(" if that character
// appears only once in the original string, or ")" if that character appears more than once in the original string. Ignore
// capitalization when determining if a character is a duplicate.
//
// USAGE: `cargo test`

use std::collections::HashMap;

fn duplicate_encode(word: &str) -> String {
    // create a hash map of characters and their counts
    // for each character of the String check the map for count and push correct bracket to result String
    let mut character_counts = HashMap::new();

    for character in word.to_lowercase().chars() {
        // update a key, guarding against the key possibly not being set
        let count = character_counts.entry(character).or_insert(0);
        *count += 1;
    }

    // store result in new String
    let mut new_word = String::new();
    for character in word.to_string().to_lowercase().chars() {
        let new_character = match character_counts.get(&character) {
            Some(i) => {
                match i {
                    1 => '(',
                    _ => ')',
                }
            },
            None => '-',
        };
        new_word.push(new_character);
    }
    new_word
}

#[test]
fn run_tests() {
  assert_eq!(duplicate_encode("din"),"(((");
  assert_eq!(duplicate_encode("recede"),"()()()");
  assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
  assert_eq!(duplicate_encode("(( @"),"))((");
}
