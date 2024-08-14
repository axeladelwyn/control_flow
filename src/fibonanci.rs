use std::io;

fn fibonacci(n: usize) -> Vec<u32> {
    let mut fib = vec![0; n + 1];
    if n > 0 {
        fib[1] = 1;
    }
    for i in 2..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    fib
}

fn main() {

    

    let n = 10; // Change this value to get more Fibonacci numbers
    let fib_sequence = fibonacci(n);
    
    // Print the entire Fibonacci sequence
    println!("Fibonacci sequence up to {}: {:?}", n, fib_sequence);
    
    // Access the nth Fibonacci number'

    println!("Enter fibonacci number");
    let mut nth = String::new(); // Change this value to access a different Fibonacci number
    io::stdin().read_line(&mut nth).expect("failed to read line");
    let nth: usize = nth.trim().parse().expect("please type a number!");

    if nth <= n {
        println!("The {}th Fibonacci number is {}", nth, fib_sequence[nth]);
    } else {
        println!("Index out of range");
    }
}
