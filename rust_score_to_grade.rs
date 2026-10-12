use std::io::{self, Write};

fn main() {
    let score: i32 = loop {
        match input("Please enter your score.").parse() {
            Ok(n) => break n,
            Err(_) => {
                println!("Invalid value.");
                continue;
            }
        }
    };

    let grade = grade(score);

    if grade == 'X' {
        println!("Score out of range.")
    } else {
        println!("Your grade is {}.", grade);
    }
}

fn grade(score: i32) -> char {
    match score {
        0..60 => 'F',
        60..70 => 'D',
        70..80 => 'C',
        80..90 => 'B',
        90..=100 => 'A',
        _ => 'X',
    }
}

fn input(promp_msg: &str) -> String {
    println!("{}", promp_msg);
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");
    s.trim().to_string()
}
