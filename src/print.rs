pub fn run(){
    //basic printing
    println!("Hello from the function");

    //Basic formatting
    println!("{} and {}",
             5 + 4, 10 * 5);

    //Positional Formating
    println!("{0} is from {1} and {0} likes to {2}",
             "Celian", "France", "code");

    //Named Formating
    println!("{name} loves {activity}",
             name = "Celian",
             activity = "code");


    // Placeholder traits
    println!("Hex: {:#X} Octal: {:#o} Binary {:#b}", 10, 10, 10);

    // Debug
    println!("{:#?}", (12, true, "hello"))
}