use std::io::{stdin, Stdin};
// use std::iter::{su}

fn _f_to_c(float_string: &str) -> f64 {
    float_string.parse::<f64>().unwrap_or(0.0)
}

fn _conversions() {
    let input: Stdin = stdin();
    let mut user_input_string = String::new();

    match input.read_line(&mut user_input_string) {
        Ok(_number_of_bytes) => {
            let user_input_string = user_input_string.trim_end();
            let user_fahrenheit = _f_to_c(user_input_string);
            let celsius_conversion: f64 = (user_fahrenheit - 32.0) / 1.8;
            println!("{user_fahrenheit}F = {celsius_conversion}C");
        }
        Err(error) => {
            println!("You may have inputed a bad value: {error}");
        }
    }
}

fn fib(number_of_fibonacci_numbers: u32) {
    let mut fibonacci_sequence: Vec<u32> = vec![0, 1];
    for n in 1..number_of_fibonacci_numbers as usize {
        fibonacci_sequence.push(fibonacci_sequence[n - 1..].iter().sum());
    }
    println!("Fib sequence: {:#?}", fibonacci_sequence);
}

fn main() {
    println!("How many fib numbers do you want to generate: ");
    let mut user_input = String::new();
    stdin().read_line(&mut user_input).unwrap();
    let user_input: &str = user_input.trim_end();

    let number_of_fibonacci_numbers: u32 = user_input.parse::<u32>().unwrap_or(2) - 1;
    fib(number_of_fibonacci_numbers);
}
