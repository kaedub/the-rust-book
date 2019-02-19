use std::io;
// use rand::Rng;

fn main() {
    // create a random unsigned 8 byte integer
    let secret = rand::random::<u8>();
    println!("Guess the number!");
    println!("Please input your guess.");
    println!("**The number is {}**", secret);

    // create mutable string
    let mut input_text = String::new();

    // read standard input
    io::stdin()
        .read_line(&mut input_text)
        .expect("Failed to read line");

    // trim white space, parse integer, and handle error
    let guess = input_text
        .trim()
        .parse()
        .expect("Expected a number");

    if secret == guess {
        println!("Correct!");
        println!("You guessed: {}", guess);
    } else {
        println!("Sorry, that was incorrect.");
        println!("You guessed: {}", guess);
        println!("The answer was: {}", secret);
    }
}