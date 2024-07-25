pub fn run() {
    let array1 = [1,2,3];
    let array2 = array1;

    println!("{:?}", (array1, array2));

    // with non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
    // You'll need to use a reference (&) to point to the resource

/*    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("{:?}", (&vec1, vec2));*/

    let vec1 = vec![1,2,3];
    let vec2 = vec1.clone();

    println!("{:?}", (vec1, vec2));
}