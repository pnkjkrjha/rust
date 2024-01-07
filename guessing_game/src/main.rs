use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("there should not be any exception");
    println!("You guessed: {guess}");
    println!("Hello, world!");
}
