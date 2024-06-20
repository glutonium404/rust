use std::io;

fn main() {
    println!("Welcome to the nth fibonacci calculator.");
    
    loop {
        println!("Please provide a valid integer to find the nth fibonacci number");
    
        let mut n: String = String::new();
    
        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");
    
        let n: u16 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your {}th fibonacci number is {}", n, nth_fib(n));

    }
}

fn nth_fib(n: u16) -> u64 {
    if n <= 2 {
        return (n - 1) as u64;
    }
    else {
        let mut sum: u64 = 1;
        let mut f1: u64 = 0;

        for _i in 0..=(n - 2) {

            let temp: u64 = sum;
            sum += f1;
            f1 = temp;
        }
        return sum;
    }
}
