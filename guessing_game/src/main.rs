use std::io;
use std::cmp::Ordering;

// use rand::Rng;

fn main() {
    // create a random unsigned 8 byte integer
    let secret_number = rand::random::<u8>();
    println!("Guess the number!");
    println!("Please input your guess.");
    println!("**The number is {}**", secret_number);

    // create mutable string
    let mut guess = String::new();

    // read standard input
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // trim white space, parse integer, and handle error
    let  guess: u8 = guess.trim().parse()
        .expect("Expected a number to be input");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
    println!("The number is {}", secret_number)
}