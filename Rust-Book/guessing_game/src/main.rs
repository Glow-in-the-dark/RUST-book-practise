// use the io (inout/output library) from the std (standard library)
use std::io;  
use rand::Rng;
use std::cmp::Ordering; // to "compare", Ordering type have variants "Less, greater, Equal"

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is:{secret_number}");

    println!("Please input your guess.");

    // variables are "by default" Immutable in RUST, so if the variables can be changed, it must be assigned "mut" (mutable)
    let mut guess = String::new();
    // the ::new, means the "new() function is an associated function of the String type"
    
    // The below can actually also be written as a "one liner"
    // io::stdin().read_line(&mut guess).expect("Failed to read line");
    io::stdin()
        .read_line(&mut guess)  // &(reference) by default is also Immutable, therefore we need to use 'mut'
        .expect("Failed to read line"); //

    println!("You guessed: {guess}");
}
