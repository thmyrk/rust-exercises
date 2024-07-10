struct Summary<'a> {
    title_summary: &'a str,
}

impl<'a> Summary<'a> {}

// function where lifetime annotation is not necessary, it is assumed automatically by compiler
fn first_character(x: &str) -> &str {
    &x[0..1]
}

// function where explicit lifetime annotation is necessary
fn longer_slice<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "word";
    println!("First character of {0} is {1}", x, first_character(x));

    let longest;
    let first_char;
    {
        let y = "longer_word";
        longest = longer_slice(x, y);
        first_char = first_character(y);
    }

    println!("The word {0} is longest", longest);
    println!("First character of that word is {0}", first_char);
}
