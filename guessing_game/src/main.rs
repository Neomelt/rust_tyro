use rand::Rng;
use std::cmp::Ordering; // Ordering is a enum type,which has three variants: Less, Greater and Equal
// cmp is a method of the PartialOrd trait,which is used to compare two values
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // create a new mutable String

        io::stdin()
            .read_line(&mut guess) // mut guess all send to the function with reference
            // reference is inalterable(couldn't change),so we need to use mut to change the value
            .expect("Failed to read line"); // rust use point to describe the method
        // this above code is one line,but we make it more readable

        // read_line() function will return a Result type,and the result data type is enum
        // Result is a enum type,which has two variants: Ok and Err
        // Result often use with match statement
        // if result return the err,expect() will crash the program
        // if result return the ok,expect() will return the value of Ok
        let guess: u32 = match guess.trim().parse() {
            // trim() will remove the whitespace and newline characters from the string
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        // match is mak match the value of guess with the value of secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
