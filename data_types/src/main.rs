mod compounds;

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    // 100,000 will overflow 16-bit unsigned int
    let mut hunned: u32 = 100_000;
    println!("Guess is {} and hunned is {}", guess, hunned);
    hunned = hunned / 113;
    println!("hunned is {}", hunned);

    // floats
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x = {}, y = {}", x, y);

    // Booleans
    let t = true;
    let f: bool = false;

    println!("true {}, false {}", t, f);

    // Chars
    let c = 'z';
    let z = '◊';
    let boom = '💥';

    println!("{} {} {}", c,z,boom);


    // Compound Types
    compounds::ex_compounds();





                    /* Notable Errors */

    /*
    The line below gives this really helpful error about printing without a string

            error: format argument must be a string literal
               --> src/main.rs:43:14
               |
            43 |     println!(c,z,boom);
               |              ^
            help: you might be missing a string literal to format with
               |
            43 |     println!("{} {} {}", c,z,boom);

    */
    // println!(c,z,boom);



    /* NUMERIC OPERATIONS NOT ALLOWED BETWEEN TYPES */

    // error[E0277]: cannot add a float to an integer
    // let sum = 5 + 10.2;

    // error[E0277]: cannot subtract `{float}` from `{integer}`
    // let difference = 95 - 4.3;

    // error[E0277]: cannot multiply `{integer}` to `{float}`
    // let product = 4.2 * 30;

    // error[E0277]: cannot divide `{float}` by `{integer}`
    // let quotient = 56.7 / 32;

    // error[E0277]: cannot mod `{integer}` by `{float}`
    // let remainder = 43 % 5.2;
}
