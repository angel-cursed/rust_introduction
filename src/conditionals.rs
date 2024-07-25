pub fn conditionals() {
    let age : u8 = 22;
    let check_id = false;
    let knows_person_age = true;

    if age >= 21 && (check_id || knows_person_age) {
        println!("You can drink!");
    }else if age < 21 && check_id{
        println!("Go away !")
    }else{
        println!("I need your ID")
    }

    // shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age)
}