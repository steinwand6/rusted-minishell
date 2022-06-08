use std::{path::Path, process::exit, str::SplitWhitespace};

pub fn buildin_exit() {
    exit(0);
}

pub fn buildin_cd(args: SplitWhitespace) -> u8 {
    let new_dir = args.peekable().peek().map_or("/", |x| *x);
    //let root = Path::new(new_dir);
    let root = Path::new(new_dir);
    if let Err(e) = std::env::set_current_dir(&root) {
        eprintln!("{}", e);
        return 1;
    }
    0
}

pub fn buildin_env() -> u8 {
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value);
    }
    0
}
