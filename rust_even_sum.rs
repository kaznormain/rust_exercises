use std::io::{self, Write};

fn main() {
    let num: i32 = loop {
        match input("Enter a number.").parse() {
            Ok(n) => break n,
            Err(_) => {
                println!("Invalid value.");
                continue;
            }
        };
    };
    sum_even(num);
}

fn sum_even(n: i32) {
    let mut buf = 0;
    for i in 0..n / 2 {
        buf += (i + 1) * 2;
    }
    println!("{}", buf);
}

fn input(promp_msg: &str) -> String {
    println!("{}", promp_msg);
    print!("> ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line.");
    s.trim().to_string()
}
