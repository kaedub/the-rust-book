fn main() {
    let number = 3;

    /* 
    if expressions... 
    yes they are if expressios
    not if statements
    */
    if number == 0 {
        println!("number is zero");
    } else if number < 4 {
        println!("number is less than four");
    } else {
        println!("number is four or greater");
    }

    // this should return 3
    let number = if 3 > 4 { 2 } else { 3 };

    println!("number is {}", number);

    loop {
        println!("if_expression returns {}", if_expression());
    }
    // Rust does not do type coercion
    // error[E0308]: mismatched types
    // if number {
    //     println!("condition was true");
    // }
}

// interesting
fn if_expression() -> u32 { if 3 > 4 { 10 } else { 12 } }