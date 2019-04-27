use std::io;

fn main() {
    let mut temperature = String::new();
    let mut choice = String::new();

    println!("Do you wish to convert from celsius [1] or fahrenheit [2]?");

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");
    let choice = choice.trim();

    println!("Please input the temperature you want to convert from");
    io::stdin().read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f32 = temperature.trim().parse()
        .expect("Failed to convert");

    // let result = if choice == 1 {
    //     temperature * 1.8 + 32
    // } else if choice == 2 {
    //     (temperature - 32) / 1.8
    // };

    let result = match choice {
        "1" => temperature * 1.8 + 32.,
        "2" => (temperature - 32.) / 1.8,
        _ => panic!("Wrong choice idiot!")
    };

    println!("The result is {:?}", result);
}
