//arrays length is fixed :(
// vector > arrays
use std::mem;

pub fn arrays() {
    let mut numbers: [i8; 5] = [1, 2, 3, 4, 5];

    numbers[3] = 20;

    println!("{:#?}", numbers);

    // Get single val
    println!("{:#?}", numbers[2]);

    println!("{:#?}", (numbers.len(), mem::size_of_val(&numbers)));

    let slice: &[i8] = &numbers[0..3];
    println!("{:#?}", slice);
}