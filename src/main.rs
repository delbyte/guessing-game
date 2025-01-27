use std::{cmp::Ordering, io};  //use the standard input output library
use rand::Rng;

//main function
fn main() {
    let secret = rand::thread_rng()
        .gen_range(1..=100);

    let mut num :i32 = 0;

    loop {   //for infinite guesses
    
        println!("Guess a number from 1-100:");   
        let mut guess = String::new();  //declare mutable variable
        
        //receive user input
        io::stdin() 
            .read_line(&mut guess)   //assign input to guess variable
            .expect("Failed to read!");   //handle exception error

        println!("You guessed: {}", guess);

        //conversion of input to integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    
        //comparison of guess to secret num
        match guess.cmp(&secret) {
            Ordering::Greater => println!("Too high!"),
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
