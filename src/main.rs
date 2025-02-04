use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("guess a number");
    let secret_number=rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");
    println!("input your guess");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");
    let guess:u32 = guess.trim().parse().expect("Please type a number");
    println!("You guess {guess}");

    match guess.cmp(&secret_number){
        Ordering::Greater => println!("too big!"),
        Ordering::Less => println!("too small!"),
        Ordering::Equal => println!("The Same"),
    }

}
