use std::io::{self, Write};

fn main() {
    let num = input("Enter a number.").parse().expect("Invalid value.");
    let num = fibonacci(num);
    println!("{}", num);
}

fn fibonacci(n: u32) -> u32 {
    loop {
        if n == 0 {
            n
        } else if n == 1 {
            n
        } else {
            (n - 1) + (n - 2)
        }
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
