use std::io;

fn main() {
    let mut number_of_terms = String::new();
    let mut first_value = String::new();

    println!("Input number of terms in the sequence");

    //Fetch user input
    io::stdin()
        .read_line(&mut number_of_terms)
        .expect("Failed to read line");

    //Check to ensure input is a number
    let number_of_terms: u32 = match number_of_terms.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Value should be a number");
            std::process::exit(1);
        }
    };

    println!("Input starting value");

    //Fetch user input
    io::stdin()
        .read_line(&mut first_value)
        .expect("Failed to read line");

    //Check to ensure input is a number
    let mut first_value: u32 = match first_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Value should be a number");
            std::process::exit(1)
        }
    };

    //Fibonacci algorithm
    let mut second_value = first_value;

    print!("{first_value}, {second_value}");

    for mut _x in 1..=number_of_terms - 2 {
        let c = first_value + second_value;
        print!(", {c}");

        first_value = second_value;
        second_value = c;

        _x += 1;
    }
}
