// practice from the rust book
// https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// notated for myself for reference


// imports
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // using import and function to generate an unsigned 32 bit integer from 1-100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // loop keyword makes an infinite loop
    loop {
        println!("Please input your guess.");

        // let makes a variable
        // mut means the variable is mutable (you can change it)
        // we define guess as a string, the :: thingy is kind of like . but not
        // defines it as a new string
        let mut guess = String::new();

        // input/output function reads the line and returns an enum 
        // this specific one returns the variables Ok or Err 
        // if it is Err then it prints "Failed to read line"
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // tries to make guess into a number (is currently a string)
        // it 'match'es are guess but first trims it (removes whitespace) and tries to 
        // parse it into an int 
        // if it's ok, then it makes guess whatever number it was able to parse 
        // if it errors, then it takes whatever error (since _ is a catch all and is like *) and continues the loop 
        // aka it goes back to the beginning and tells you to give it a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // matching the guess again with the secret number 
        // this returns another enum which will be one of the below
        // it will be less, greater, or equal
        match guess.cmp(&secret_number) {
            // compares the enum that it got above by comparing the two numbers
            // does the action that matches with the enum that it got
            // in this case it prints if the number if too big or too small
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // if it got equal, then it prints that you win and also breaks the loop to 
            // finish the program and stop it
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}