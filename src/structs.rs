struct Color {
        red: u8,
        green: u8,
        blue: u8,
        alpha: Option<u8>,
    }

// Tuple struct
struct Colour(u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn new(first_name: &str, last_name: &str, age: u8) -> Person {
        Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age: age,
        }
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    fn birthday(&mut self) {
        self.age += 1;
    }

    fn to_tuple(&self) -> (String, String, u8){
        (self.first_name.clone(), self.last_name.clone(), self.age)
    }
}

pub fn run() {
    let mut color = Color {
        red: 255,
        green: 0,
        blue: 0,
        alpha: None,
    };

    color.red = 50;

    match color.alpha{
        Some(value) => println!("Color:\n R: {}\n G: {}\n B: {}\n A: {}", color.red, color.green, color.blue, value),
        None => println!("Color:\n R: {}\n G: {}\n B: {}", color.red, color.green, color.blue),

    }

    let mut colour = Colour(255,0,60);

    colour.1 = 35;

    println!("Colour {} {} {}", colour.0, colour.1, colour.2);

    let mut p = Person::new("John","Smith", 54);

    p.set_last_name("Doe");
    p.birthday();

    let _full_name = p.get_full_name();

    println!("{:#?}", p.to_tuple());
}