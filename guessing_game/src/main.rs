use rand::prelude::*;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Get an RNG:
    let mut rng = rand::rng();
    let secret_number = rng.random_range(1..=100);
    // println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("{}. Please type a number! \n Lets try again \n\n", err);
                continue;
            }
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!!"),
            Ordering::Equal => {
                
                println!("That's Right! You win");
                break;
            }
        }
    }
} // end func
