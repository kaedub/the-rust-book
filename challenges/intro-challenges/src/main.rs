// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn fahr_to_cels(t: f32) -> f32 {
    (t - 32.0) * 5.0 / 9.0
}

fn fib_to(n: i32) -> i64 {
    let mut previous = 0;
    let mut current = 1;
    let mut tmp;
    for _i in 2..n+1 {
        tmp = current + previous;
        previous = current;
        current = tmp;
    }
    current
}

// >>> def fib_to(n):
// ...     fibs = [0, 1]
// ...     for i in range(2, n+1):
// ...         fibs.append(fibs[-1] + fibs[-2])
// ...     return fibs

fn main() {
    let mut ftemp: f32 = 32.0;
    println!("{}˚F -> {}˚C", ftemp, fahr_to_cels(ftemp));
    ftemp = 212.0;
    println!("{}˚F -> {}˚C", ftemp, fahr_to_cels(ftemp));

    let mut n = 7;
    println!("#{} fibonacci number is {}", n, fib_to(n));
    n = 13;
    println!("#{} fibonacci number is {}", n, fib_to(n));
    n = 21;
    println!("#{} fibonacci number is {}", n, fib_to(n));
}
