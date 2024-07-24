//vector is like an arrays but growable
// vector > arrays
use std::mem;

pub fn vectors_intro() {

    let mut numbers: Vec<i8> = vec![1, 2, 3, 4, 5];

    numbers[3] = 20;

    println!("{:#?}", numbers);

    // Get single val
    println!("{:#?}", numbers[2]);

    println!("{:#?}", (numbers.len(), mem::size_of_val(&numbers)));

    let slice: &[i8] = &numbers[0..3];
    println!("{:#?}", slice);

    numbers.append(&mut vec![5,4,6,5]);
    println!("{:#?}", numbers);

    numbers.push(5);
    println!("{:#?}", numbers);

    numbers.pop();
    numbers.remove(4);

    // remove a value
    numbers.retain(|&x| x != 20);

    println!("{:?}", numbers);

    for x in numbers.iter() {
        println!("{x}");
    }

    for x in numbers.iter_mut() {
        *x *= 2
    }

    println!("{:?}", numbers);
}