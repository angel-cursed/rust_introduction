use std::io;
pub fn run() {
    let mut input = String::new();

    println!("Enter a text:");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Removes newline
    let input = input.trim();

    println!("You entered: {}", input);
}