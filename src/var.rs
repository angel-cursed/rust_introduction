pub fn run(){
    let name = "brad";
    let mut age = 13;

    println!("Hello my name is {} and I'm {} years old!", name, age);

    age += 1;
    println!("I'm now {} years old", age);

    // define constants
    const ID: i32 = 64;
    println!("My ID is {ID}");

    //multiple assign

    let (your_name, your_age) = ("Brad", 18);

    println!("Hello, {}! You are {} years old", your_name, your_age);

}

// pub fn birthday(age : pointer) {
//     *age += 1;
//     println!("I'm now {} years old", *age);
// }