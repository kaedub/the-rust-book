fn ex_identifiers() {
    println!("<- Identifiers ->");

    // this is not an appropriate identifier
    let _ = "_";
    let __ = 10;
    // this is a raw identifier, 'match' is a keyword in Rust
    let r#match = "match";
    println!("match: {}", r#match);
    println!("__: {}", __);
    /* This will throw an error, expected expression
       "_" is not appropriate identifier */
    // println!("_: {}", _);
}

fn ex_mutability() {
    println!("<- Mutability ->");
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

fn ex_constants() {
    println!("<- Constants ->");
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS is a const of {}", MAX_POINTS);
}

fn ex_shadowing() {
    println!("<- Shadowing ->");
    let val = 5;
    println!("val is {}", val);

    let val = val + 1;
    println!("val is now {}", val);

    let val = val * 2;
    println!("val is now {}", val);

    let spaces = "   ";
    println!("spaces is {}.", spaces);
    let spaces = spaces.len();
    println!("spaces is now {}.", spaces);


    // warning: variable does not need to be mutable
    let mut mut_spaces = "   ";
    println!("mut_paces is {}.", mut_spaces);
    let mut_spaces = mut_spaces.len();
    println!("mut_paces is now {}.", mut_spaces);
}

fn main() {
    ex_identifiers();
    ex_mutability();
    ex_constants();
    ex_shadowing();
}
