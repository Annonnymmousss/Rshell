#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main() {
    // TODO: Uncomment the code below to pass the first stage

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let builtin = vec!["type", "echo", "exit", "pwd"];
        let mut command = String :: new();
        

        io::stdin()
            .read_line(&mut command)
            .expect("Unable to read the input");

        let parts = &command.trim().split_whitespace();
        let mut args: Vec<&str> = parts.clone().collect(); 
        args.remove(0);

        let first_word = &command.split_whitespace().next();
        let first_word_str = first_word.unwrap_or("");

        if command.trim() == "exit"{
            break;
        }else if first_word == &Some("echo") {
            let trimmed = command.trim();
            let (_,rest) = trimmed.split_once("echo ").unwrap_or(("",&trimmed));
            println!("{}",rest);
            continue;
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
                continue;
            }
        
        }else if first_word == &Some("pwd") {
            if let Ok(cwd) = env::current_dir() {
                println!("{}", cwd.display());
            }
            continue;
        }else{
            if command_exists(first_word_str){
                let output = Command::new(first_word_str)
                    .args(&args)
                    .output()
                    .expect("failed to execute process");
                print!("{}", String::from_utf8_lossy(&output.stdout));
                continue;
            }else {
                println!("{}: command not found",command.trim());
                continue;
            }
        }
    }


    fn command_exists(cmd: &str) -> bool {
        if let Ok(paths) = env::var("PATH") {
            for dir in paths.split(':') {
                let full_path = Path::new(dir).join(cmd);
                if full_path.exists() {
                    if let Ok(meta) = full_path.metadata() {
                        if meta.permissions().mode() & 0o111 != 0 {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}