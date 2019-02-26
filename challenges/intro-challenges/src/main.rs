// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.


fn main() {
    xmas_song();
    
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

fn xmas_song() {
    let gifts = ["drummers drumming","pipers piping",
        "lords a leaping","ladies dancing","maids a milking",
        "swans a swimming","geese a laying","gold rings",
        "calling birds","French hens","turtle doves",
        "partridge in a pear tree"
    ];
    for day in 0..12 {
        println!("On the {} day of Christmas,\nMy true love gave to me", day);
        for line in 0..day {
            let i: usize = day - line;
            println!("{} {}", i, gifts[line])
        }
    }
}