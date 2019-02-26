// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.

fn fahr_to_cels(t: f32) -> f32 {
    (t - 32.0) * 5.0 / 9.0
}

fn main() {
    let mut ftemp: f32 = 32.0;
    println!("{}˚F -> {}˚C", ftemp, fahr_to_cels(ftemp));
    ftemp = 212.0;
    println!("{}˚F -> {}˚C", ftemp, fahr_to_cels(ftemp));

}
