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

        // Input stream to accept temperature and save it to the variable temperature
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        // This block trims and parses the temperature to an f64 variable and binds it to temperature
        let temperature: f64 = match temperature.trim().parse() {
            // If everything is Ok, the evaluated expression should be returned and binded to temperature
            // Everything being Ok means the input is something that can reasonably be converted to an f64
            Ok(temp) => temp,

            // If the input is something that can be parsed to an f64, show a message and skip the rest of the code
            Err(_) => {
                println!("Enter a valid temperature value");
                continue;
            }
        };

        println!("Is the temperature in (C)elsius or (F)ahrenheit?");
        // Get the unit and bind it to the variable
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
        println!("Convert again?(y/n)");
        let mut convert_again = String::new();
        io::stdin()
            .read_line(&mut convert_again)
            .expect("Failed to read line");
        let convert_again = convert_again.trim().chars().next();
        if convert_again == Some('N') || convert_again == Some('n') {
            break;
        }
        // println!("{temperature}");
    }

    fn fahrenheit_to_celsius(temp_in_fahrenheit: f64) -> f64 {
        (temp_in_fahrenheit * 5.0 / 9.0) - 32.0
    }
    fn celsius_to_fahrenheit(temp_in_celsius: f64) -> f64 {
        (temp_in_celsius * 9.0 / 5.0) + 32.0
    }
}
