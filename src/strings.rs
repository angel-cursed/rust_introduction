pub fn strings_intro() {
    let mut hello = String::from("Hello ");
    let primitive = "hello";

    // Get length works for str and String

    println!("{:#?}", (hello.len(), hello.clone(), primitive.len()));

    // Push add a char at the end of the string
    hello.push('W');
    hello.push_str("orld");

    println!("Capacity {} Is empty {}", hello.capacity(), hello.is_empty()); // Works for &str

    println!("Contains 'World' {}", hello.contains("World")); // Works for &str

    println!("Replace 'Hello' with 'Hi' {}", hello.replace("Hello", "Hi")); // Works for &str

    for word in hello.split_whitespace() {// Works for &str
        println!("{}", word);
    }

    for word in hello.split("") {// Works for &str
        println!("{}", word);
    }


    // Create a string whit capacity
    let mut s = String::with_capacity(10);
    s.push('a');

    //assertion testing
    assert_eq!(1, s.len()); // error if left is not equal to right

    println!("{}", s.capacity());
}