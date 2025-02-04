use std::io;
use rand::Rng;
fn main() {
    println!("guess a number");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("input your guess");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");
    println!("You guess {guess}")
}
