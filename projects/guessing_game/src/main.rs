use std::io;

fn main() {
    //Sending an input, the ! indicates a macro instead of a method call.
    println!("Guess a number");
    println!("Please input your guess: ");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Couldn't read input");

    println!("You guessed: {}", guess);

}
