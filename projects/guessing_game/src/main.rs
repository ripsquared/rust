extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    //Sending an input, the ! indicates a macro instead of a method call.
    println!("Guess a number");

    //Generate a secret random number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    println!("Please input your guess: ");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Couldn't read input");

    println!("You guessed: {}", guess);

}
