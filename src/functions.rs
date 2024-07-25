pub fn run(){
    greeting("hi", "Marc");

    let _sum = add(8,2);
    println!("{}", add(8,4));

    // Closure
    let  c = 6;
    let add_nums = |a: i32, b: i32| return a + b + c;

    println!("{}", add_nums(6,46))
}

fn greeting(greet : &str, name : &str){
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}