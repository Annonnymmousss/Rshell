#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;

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
            let cmd = rest.trim();
            let paths = env::var("PATH").unwrap();
            let mut found = false;
            if builtin.contains(&rest) {
                println!("{} is a shell builtin",&rest);
                continue;
            }
            for dir in paths.split(':') {
                let full_path = Path::new(dir).join(cmd);

                if full_path.exists() {
                    let meta = full_path.metadata().unwrap();
                    if meta.permissions().mode() & 0o111 != 0 {
                        println!("{cmd} is {}", full_path.display());
                        found = true;
                        break;
                    }
                }
            }
            if !found {
                println!("{}: not found",cmd);
            }
        
        }else {
            println!("{}: command not found",command.trim());
        }
    }
}