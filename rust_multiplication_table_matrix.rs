fn main() {
    let n = 40;
    for i in 0..=n {
        if i == 0 {
            print!("    ");
            for j in 1..=n {
                print!("|{:>4}", j);
            }
            println!("");
            print!("----");
            for j in 1..=n {
                print!("|----");
            }
            println!("");
        } else {
            for j in 0..=n {
                if j == 0 {
                    print!("{:>4}", i);
                } else {
                    print!("|{:>4}", i * j);
                }
            }
            println!("");
        }
    }
}
