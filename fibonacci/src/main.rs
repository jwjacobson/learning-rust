// Fibonacci number calculator

use std::io;

fn main() {
    // Take user input, run Fibonacci on it, then print the result.
    println!("What degree of the Fibonacci sequence should I print?");

    let mut n = String::new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u128 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    println!("{}", fibonacci(n))
}

fn fibonacci(n: u128) -> u128 {
    // Return the nth Fibonacci number
    // Can be faster once I learn about hash maps
    if n == 0 || n == 1{
        return n
    }
    else {
        return fibonacci(n - 2) + fibonacci(n - 1)
    }
}
