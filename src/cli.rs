pub fn run() {
    let args_vec: Vec<String> = std::env::args().collect();
    let mut args = std::collections::HashMap::new();

    // println!("{}", command);
    for (index, arg) in args_vec.clone().into_iter().enumerate() {
        if arg.starts_with("--") {
            if index + 1 < args_vec.len() {
                args.insert(arg, args_vec[index+1].clone());
            }else{
                args.insert(arg, "".to_string());
            }
        }
    }

    let mut name = None;
    let mut hello = None;

    for (key, value) in &args {
        if key.to_string() == "--name"{
            name = Some(value.clone());
        }
        if key.to_string() == "--hello"{
            hello = Some(value.clone());
        }
        if key.to_string() == "--h"{
            println!("Usage: rust_introduction\n--name <name>\n--hello <hello_message>");
            std::process::exit(0);
        }
    }

    println!("{} {}",hello.unwrap_or("".to_string()), name.unwrap_or("".to_string()));
}