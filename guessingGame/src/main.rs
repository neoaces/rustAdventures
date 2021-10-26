use std::io;
use rand::Rng;
fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // initialized generator with rand::thread_rng()
    // then used .gen_range() to get a number 
    //   - note: .. is for exclusive range literal

    let secret_number = rand::thread_rng()
        .gen_range(1..101);

    let mut guess = String::new();

    // will throw an error if .expect() isn't present
    // because Err is not handled (enumeration *variant* of a Result)
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");

    // template literals
    println!("You guessed: {}", guess);
    println!("Secret number was: {}", secret_number);

}
