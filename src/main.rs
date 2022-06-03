use std::{
    io::{stdin, stdout, Write},
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

        let command = input.trim();

        let mut child = Command::new(command).spawn().unwrap();

        // don't accept another command until this one completes
        _ = child.wait();
    }
}
