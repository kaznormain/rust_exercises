fn main() {
    let numbers = [12, 7, 25, 3, 18, 42, 9];
    let max = find_max(numbers);
    println!("{}", max);
}

fn find_max(numbers: [i32; 7]) -> i32 {
    let mut max = 0;
    for n in numbers {
        if n > max {
            max = n;
        }
        println!("n: {}\nmax: {}", n, max);
    }
    max
}
