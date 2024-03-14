fn main() {
    assert_eq!(int32_to_ip(2154959208), "128.114.17.104");
    assert_eq!(int32_to_ip(2149583361), "128.32.10.1");
    assert_eq!(int32_to_ip(2239), "0.0.8.191");
    assert_eq!(int32_to_ip(0), "0.0.0.0");
    println!("Tests passed");
}

fn int32_to_ip(int: u32) -> String {
    let binary_string = decimal_to_binary(int);

    let octet_decimals = translate_binary_to_octet_decimals(binary_string);
    octet_decimals.join(".")
}

fn decimal_to_binary(mut decimal: u32) -> String {
    let mut binary = String::new();

    while decimal > 0 {
        binary = format!("{}{}", (decimal % 2).to_string(), binary);
        decimal = decimal / 2;
    }

    binary = format!("{}{}", "0".repeat(32 - binary.len()), binary);

    binary
}

fn translate_binary_to_octet_decimals(mut binary_string: String) -> Vec<String> {
    let mut octet_decimals: Vec<String> = Vec::new();

    while binary_string.len() > 0 {
        let octet = match binary_string.get(0..8) {
            Some(octet_string) => octet_string,
            None => &binary_string,
        };

        match u32::from_str_radix(octet, 2) {
            Ok(octet_decimal) => octet_decimals.push(octet_decimal.to_string()),
            Err(error) => panic!("{error}"),
        };

        binary_string = if binary_string.len() > 8 {
            binary_string[8..].to_string()
        } else {
            "".to_string()
        };
    }

    octet_decimals
}
