use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();

    println!("Your secret number is: {secret_number}");
    
    io::stdin().read_line(&mut guess).expect("failed to read this line");

    let guess: u32 = guess.trim().parse().expect("Please Type a Number!");

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small :("),
        Ordering::Equal => println!("You Win!!!!"),
        Ordering::Greater => println!("Too Big :(")
    }
}
