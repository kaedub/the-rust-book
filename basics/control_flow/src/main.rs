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

    println!("if_expression returns {}", if_expression());
    

    println!("ex_loop_with_break returns {}", ex_loop_with_break());
    println!("ex_loop_while returns {}", ex_loop_while());
    ex_loop_for();
    ex_rev_for();

    // infite loop
    // loop {
    //     println!("if_expression returns {}", if_expression());
    // }


            /* Notable Errors */

    // Rust does not do type coercion
    // error[E0308]: mismatched types
    // if number {
    //     println!("condition was true");
    // }
}

fn ex_rev_for() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn ex_loop_for() {
    let a = [1,2,3,4];

    for element in a.iter() {
        println!("a[i]={}", element)
    }
}

fn ex_loop_while() -> u8 {
    let mut number = 3;
    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Liftoff!");
    number
}

// fn ex_while() {
//     let mut number = 3;

    // while number !=  {
    //     println!("{}!", number);
    //     number = number - 1;
    // }
    // println!("Liftoff...");
// }

fn ex_loop_with_break() -> u32 {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {} and counter is {}", result, counter);
    assert_eq!(result, 20);
    return counter;
}

// interesting
fn if_expression() -> u32 { if 3 > 4 { 10 } else { 12 } }