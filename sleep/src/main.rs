use std::{env, process, time::Duration};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: <number>");
        process::exit(5);
    }
    match args.get(1) {
        Some(_arg) => {
            let seconds: u64 = match args[1].parse(){
                Ok(n) => n,
                Err(_) => {
                    eprintln!("Not an Int");
                    process::exit(1);
                }
            };
            std::thread::sleep(Duration::from_secs(seconds));
        },
        None => {},
    }
}
