#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let builtin = vec!["type", "echo", "exit"];

        let mut command = String :: new();
        io::stdin()
            .read_line(&mut command)
            .expect("Unable to read the input");

        let first_word = &command.split_whitespace().next();

        if command.trim() == "exit"{
            break;
        }else if first_word == &Some("echo") {
            let trimmed = command.trim();
            let (_,rest) = trimmed.split_once("echo ").unwrap_or(("",&trimmed));
            println!("{}",rest);
        }else if first_word == &Some("type"){
            let trimmed = command.trim();
            let (_,rest) = trimmed.split_once("type ").unwrap_or(("",&trimmed));
            if builtin.contains(&rest) {
                println!("{} is a shell builtin",&rest);
            }else {
                println!("{}: not found",&rest);
            };
        }else {
            println!("{}: command not found",command.trim());
        }
    }
}
