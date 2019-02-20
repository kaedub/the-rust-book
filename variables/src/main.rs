fn identifiers() {
    // this is not an appropriate identifier
    let _ = "_";
    let __ = 10;
    // this is a raw identifier, 'match' is a keyword in Rust
    let r#match = "match";

    println!("Identifiers\n/*");

    println!("match: {}", r#match);
    println!("__: {}", __);

    /* This will throw an error, expected expression
       "_" is not appropriate identifier */
    // println!("_: {}", _);
    println!("*/")
}

fn main() {
    identifiers();

    let x = 5;
    let mut y = 10;
    println!("x is {}", x);
    
    // This throws error, cannot assign twice to immutable variable
    // x = 6;
    // x += 1;
    
    // "y" is mutable so it can be reassigned

    println!("x is {}", x);
    println!("y is {}", y);
    y = 12;
    println!("y is now {}", y);

}
