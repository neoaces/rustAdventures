use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // initialized generator with rand::thread_rng()
    // then used .gen_range() to get a number 
    //   - note: .. is for exclusive range literal

    let secret_number: i32 = rand::thread_rng()
        .gen_range(1..101);

    let mut guess = String::new();

    // will throw an error if .expect() isn't present
    // because Err is not handled (enumeration *variant* of a Result)
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    // explaination of guess formatting
    // .trim(): takes off the whitespace of the string because when enter is pressed after the input is done, a \n (newspace) is added
    // .parse::<i32>(): a literal way of saying to parse the string into an i32 type
    // .expect(): handles the Result variant and the possible error

    let guess: i32 = guess
        .trim()
        .parse::<i32>()
        .expect("Please type a number!");

    // template literals
    println!("You guessed: {}", guess);
    
    // compares guess to &secret_number and from the enum Ordering, handles the result variant to a certain message
    // reference note: similar to switch statements without the cases
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Correct!"),
    }

}
