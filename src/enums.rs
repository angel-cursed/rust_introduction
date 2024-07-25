enum Movement{
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement){
    match m{
        Movement::Up => println!("Move Up"),
        Movement::Down => println!("Move Down"),
        Movement::Left => println!("Move Left"),
        Movement::Right => println!("Move Right"),
    }
}

pub fn run(){
    move_avatar(Movement::Up);
    move_avatar(Movement::Down);
    move_avatar(Movement::Left);
    move_avatar(Movement::Right);
}