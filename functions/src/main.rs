fn main() {
    println!("Hello, world!");
    another_function(22, 'h');
    // error: mismatched types
    // another_function("22", 4);

    let a = 3;
    let x: f32 = 23.4;
    let y: f32 = 11.1;

    println!("x * y is {}", multiplyf(x,y));
    println!("a * 5 is {}", multiply_int_by_five(a));
}

fn another_function(x: i32, c: char) {
    println!("Hello from over here");
    println!("x is {}, c is '{}'", x, c);
}

fn five() -> i32 { 5 }

fn multiplyf(a: f32, b: f32)  -> f32{ a*b }

fn multiply_int_by_five(a: i32) -> i32 {
    // return and the semi-colon are optional on last line
    return a * five();
}