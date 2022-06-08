mod buildin;

use std::{
    io::{stdin, stdout, Write},
    process::{Child, Command, Stdio},
};

use crate::buildin::{buildin_cd, buildin_env, buildin_exit};

fn main() {
    loop {
        // use the '>' character as the prompt
        // need to explicitly flush this to ensure it prints before read_line
        print!("> ");
        _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    buildin_cd(args);
                    previous_command = None;
                }
                "exit" => buildin_exit(),
                "env" => _ = buildin_env(),
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });
                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };
                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => previous_command = Some(output),
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }
        if let Some(mut final_command) = previous_command {
            _ = final_command.wait();
        }
    }
}
