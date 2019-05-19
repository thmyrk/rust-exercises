// https://www.codewars.com/kata/decode-the-morse-code/train/rust
// In this kata you have to write a simple Morse code decoder
//
// USAGE: `cargo test`
use std::collections::HashMap;

struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    fn decode_morse(&self, encoded: &str) -> String {
        let mut result = String::new();

        for word in encoded.trim().split("   ") {
            for morse_character in word.split(" ") {
                result.push_str(
                    match self.morse_code.get(&morse_character.to_string()) {
                        Some(ascii_character) => ascii_character,
                        None => "",
                    }
                );
            }
            result.push(' ');
        }

        result.trim_end().to_string()
    }
}

#[test]
fn run_tests() {
    let morse_decoder = MorseDecoder {
        morse_code: [("-..", "D"), ("...--", "3"), ("..-", "U"), ("...", "S"), ("..---", "2"), (".....", "5"),
                     (".-", "A"), ("-.-", "K"), ("-.", "N"), ("--.-", "Q"), ("--.", "G"), ("--..", "Z"),
                     ("....-", "4"), (".-...", "&"), ("----.", "9"), ("...-", "V"), ("--...", "7"), (".-.-.-", "."),
                     ("-....", "6"), (".--.", "P"), ("-...", "B"), ("---..", "8"), (".-.", "R"), (".----.", "\'"),
                     ("-.-.--", "!"), ("-.--.", "("), ("-..-.", "/"), (".---", "J"), ("-....-", "-"), ("-.-.", "C"),
                     ("...---...", "SOS"), (".-..", "L"), ("-----", "0"), ("-.--", "Y"), ("---", "O"), (".----", "1"),
                     (".--.-.", "@"), ("..", "I"), ("-.-.-.", ";"), ("-...-", "="), ("--", "M"), (".-.-.", "+"),
                     ("..--.-", "_"), ("....", "H"), ("--..--", ","), ("-.--.-", ")"), (".", "E"), ("..-.", "F"),
                     (".--", "W"), (".-..-.", "\""), ("..--..", "?"), ("-..-", "X"), ("...-..-", "$"), ("---...", ","),
                     ("-", "T")].iter()
                                .map(|tuple| (tuple.0.to_string(), tuple.1.to_string()))
                                .collect()
    };

    assert_eq!(morse_decoder.decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
    assert_eq!(morse_decoder.decode_morse("   "), "");
    assert_eq!(morse_decoder.decode_morse("      ...---... -.-.--   - .... .   --.- ..- .. -.-. -.-   -... .-. --- .-- -.   \
                                          ..-. --- -..-   .--- ..- -- .--. ...   --- ...- . .-.   - .... .   .-.. .- --.. -.--   \
                                          -.. --- --. .-.-.-     "),
                                           "SOS! THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.");
}
