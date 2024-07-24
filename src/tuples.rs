// tuple max 12 elements

pub fn tuples_intro() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 127);

    println!("{} is from {}, and he is {}", person.0, person.1, person.2);
}