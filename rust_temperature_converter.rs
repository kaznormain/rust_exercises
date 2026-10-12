use std::io::{self, Write};

const SEP: &str = " =======================";

fn main() {
    println!("{}", SEP);
    println!("Temperature Converter.");
    loop {
        let opt = loop {
            println!("{}", SEP);
            println!("Choose an option: \n1) °C to °F \n2) °F to °C \nq) Quit");
            print!("> ");
            io::stdout().flush().unwrap(); // flush the buffer

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Fail to read line!");

            let input = input.trim().to_lowercase();

            match input.as_str() {
                "q" => return,
                "1" | "2" => break input,
                _ => {
                    println!("{}", SEP);
                    println!("Invalid option.");
                }
            }
        };

        let temp = loop {
            println!("{}", SEP);
            println!("Enter the temperature.");
            print!("> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Fail to read line!");

            let input: f64 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("{}", SEP);
                    println!("Invalid value.");
                    continue;
                }
            };
            break input;
        };

        let resault = match opt.as_str() {
            "1" => ("°C", celsius_to_fahrenheit(temp), "°F"),
            "2" => ("°F", fahrenheit_to_celsius(temp), "°C"),
            _ => unreachable!(),
        };
        let temp = (temp * 1000.0).round() / 1000.0;
        println!("{}", SEP);
        println!("{} {} = {} {}", temp, resault.0, resault.1, resault.2);
    }
}

fn celsius_to_fahrenheit(value: f64) -> f64 {
    ((value * 9.0 / 5.0 + 32.0) * 1000.0).round() / 1000.0
}

fn fahrenheit_to_celsius(value: f64) -> f64 {
    (((value - 32.0) * 5.0 / 9.0) * 1000.0).round() / 1000.0
}
