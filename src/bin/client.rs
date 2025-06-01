use std::io;
use tkrp::client::is_prime_rpc;

fn main() {
    println!("Please enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number: u16 = input
        .trim()
        .parse()
        .expect("Please enter a valid number (0–65535)");

    if is_prime_rpc(number) {
        println!("{} is prime", number);
    } else {
        println!("{} is not prime", number);
    }
}
