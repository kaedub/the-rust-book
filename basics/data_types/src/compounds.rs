pub fn ex_compounds() {
    // tuple 
    let tup: (i32, f32, u8) = (500, 3.2, 1);
    println!("tuple {:?}", tup);
    println!("1: {}, 2: {}, 3: {}", tup.0, tup.1, tup.2);

    // array
    let a = [1,2,3,4];
    println!("array: {:?}", a);

    // error[E0308]: mismatched types, expected an array with a fixed size of 5 elements
    // let a2: [i32; 5] = [1,2,3,4,5,6];

    let a2: [i32; 3] = [1,2,3];
    println!("a2: {} {} {} ", a2[0], a2[1], a2[2]);

    let second = a2[1];
    let third = a2[2];
    // error: index out of bounds: the len is 3 but the index is 4
    // let fifth = a2[4];
    println!("second {}, third {}", second, third);

}