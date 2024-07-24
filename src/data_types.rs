pub fn run(){
    let _x = 1;

    let _y = 2.5;

    let _z : i128 = 64526265726727;

    let boolean : bool = false;

    println!("{}", boolean);

    let boolean = !boolean;

    println!("{}", boolean);

    let _is_greater = 10 == 5;

    let _char = 'c';

    let face = "\u{1F600} \u{1F602}";

    println!("Face: {}", face);

    //find max size

    println!("Max i8 {}", std::i8::MAX);
    println!("Max i16 {}", std::i16::MAX);
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i64 {}", std::i64::MAX);
    println!("Max i128 {}", std::i128::MAX);

    println!("Max u8 {}", std::u8::MAX);
    println!("Max u16 {}", std::u16::MAX);
    println!("Max u32 {}", std::u32::MAX);
    println!("Max u64 {}", std::u64::MAX);
    println!("Max u128 {}", std::u128::MAX);
}