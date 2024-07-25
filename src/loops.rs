

pub fn run() {
    let mut count: i8 = 0;

    // just infinite loop
    loop {
        count += 1;
        if count == 20{
            println!("{count}");
            break;
        }
    }

/*    count = 0;
    while count <= 100{
        if count % 15 == 0{
            println!("Fizz Buzz! {count}");
        }else if count % 5 == 0{
            println!("Buzz! {count}");
        }else if count % 3 == 0{
            println!("Fizz! {count}");
        }
        count += 1;
    }*/

    for x in 0..100{
        if x % 15 == 0{
            println!("Fizz Buzz! {}", x);
        } else if x % 5 == 0{
            println!("Buzz! {}", x);
        } else if x % 3 == 0{
            println!("Fizz! {}", x);
        } else{
            println!("{}", x);
        }
    }
}