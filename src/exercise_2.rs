// Simple cmdline app to convert temperature between Celsius and Fahrenheit
// 1. Prompt for input
// 2. Parse the input
// 3. Perform conversion
// 4. Display results
use std::io;

pub fn run() {
    println!("Welcome!");

    loop {
        println!("Input temperature: ");
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: f64 = match temperature.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Enter a valid temperature value");
                continue;
            }
        };

        println!("Is the temperature in (C)elsius or (F)ahrenheit?");
        let mut celsius_or_fahrenheit = String::new();
        io::stdin()
            .read_line(&mut celsius_or_fahrenheit)
            .expect("msg");

        let celsius_or_fahrenheit: Option<char> = celsius_or_fahrenheit.trim().chars().next();

        match celsius_or_fahrenheit {
            Some('C') | Some('c') => {
                let result = celsius_to_fahrenheit(temperature);
                println!("The result is: {:.2}", result);
            }
            Some('F') | Some('f') => {
                let result = fahrenheit_to_celsius(temperature);
                println!("The result is: {:.2}", result);
            }
            _ => {
                println!("You have to choose which temp do you want to convert to");
                continue;
            }
        }
        println!("Convert agaim?(y/n)");
        let mut convert_again = String::new();
        io::stdin()
            .read_line(&mut convert_again)
            .expect("Failed to read line");
        let convert_again = convert_again.trim().chars().next();
        if convert_again == Some('N') || convert_again == Some('n') {
            break;
        }
    }
    // println!("{temperature}");
}

fn fahrenheit_to_celsius(temp_in_fahrenheit: f64) -> f64 {
    (temp_in_fahrenheit * 5.0 / 9.0) - 32.0
}
fn celsius_to_fahrenheit(temp_in_celsius: f64) -> f64 {
    (temp_in_celsius * 9.0 / 5.0) + 32.0
}
