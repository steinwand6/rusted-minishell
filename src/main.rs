use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        // use the '>' character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                // default to '/' as new directory if one was not provided
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => return,
            command => {
                let child = Command::new(command).args(args).spawn();

                match child {
                    Ok(mut child) => {
                        _ = child.wait();
                    }
                    Err(e) => eprintln!("{}", e),
                }
            }
        }
    }
}
