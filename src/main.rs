use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut input = String::new();
        stdin().read_line(&mut input)?;

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().expect("No Command detected");
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("{}", e)
                }
            }
            "exit" => return Ok(()),
            command => {
                let mut child = Command::new(command).args(args).spawn()

                match child {
                    Ok(mut child) => { child.wait(); },
                    Err(e) => eprintln!("{}", e),
                };

            }
        }
    }
}
