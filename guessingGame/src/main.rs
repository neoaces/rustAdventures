use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // initialized generator with rand::thread_rng()
    // then used .gen_range() to get a number 
    //   - note: .. is for exclusive range literal
    let secret_number: i32 = rand::thread_rng()
    .gen_range(1..101);
    
    // loops until either the match statement is met, or 3 tries is up.
    loop {
        // initialization of variable and input directive
        let mut guess = String::new();
        println!("Please input your guess.");

        // will throw an error if .expect() isn't present
        // because Err is not handled (enumeration *variant* of a Result)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // explaination of guess formatting
        // .trim(): takes off the whitespace of the string because when enter is pressed after the input is done, a \n (newspace) is added
        // .parse::<i32>(): a literal way of saying to parse the string into an i32 type
        // match: to handle any non-number answers, use a match statement for the Enum variants of guess.trim().parse()
        // note: continue means to continue with the next loop iteration
        
        // if match replaced => .expect(): handles the Result variant and the possible error

        let guess: i32 = 
            match guess.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("You guessed: {}", guess);
        
        // compares guess to &secret_number and from the enum Ordering, handles the result variant to a certain message
        // reference note: similar to switch statements without the cases
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            },
        }
    }
}
