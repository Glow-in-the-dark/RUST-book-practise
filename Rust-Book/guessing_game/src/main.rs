// use the io (inout/output library) from the std (standard library)
use std::io;  
use rand::Rng;
use std::cmp::Ordering; // to "compare", Ordering type have variants "Less, greater, Equal"

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is:{secret_number}");

    // keep looping until 'break'
    loop {
        println!("Please input your guess.");

        // variables are "by default" Immutable in RUST, so if the variables can be changed, it must be assigned "mut" (mutable)
        let mut guess = String::new();
        // the ::new, means the "new() function is an associated function of the String type"
    
        // The below can actually also be written as a "one liner"
        // io::stdin().read_line(&mut guess).expect("Failed to read line");
        io::stdin()
            .read_line(&mut guess)  // &(reference) by default is also Immutable, therefore we need to use 'mut'
            .expect("Failed to read line"); //

        /* //while this work, it will break and crash, due to .expect(),

        //shadowing -> lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
        // guess in the expression refers to the original guess variable that contained the input as a string. 
        // trim() method on a String instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32
            // The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, guess looks like this: 5\n
        // The parse method on strings converts a string to another type. (in this case a u32 number)
        // We need to tell Rust the exact number type we want by using let guess: u32.
        // The colon (:) after guess tells Rust we’ll annotate the variable’s type
        */

        // to make it handle the previous invalid input,
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // t will return an Ok value that contains the resultant number. That Ok value will match the first arm’s pattern, and the match expression will just return the num value that parse produced and put inside the Ok value. That number will end up right where we want it in the new guess variable we’re creating.
            Err(_) => {
                println!("please put in numbers only");
                continue; 
            },
            // '_' (underscore) here. is a catchall value.
            // in this example, we’re saying we want to match all Err values, no matter what information they have inside them. So the program will execute the second arm’s code, continue                    
        };

        println!("You guessed: {guess}");

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
