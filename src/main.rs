#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main() {

    loop{
        print!("$ ");
        io::stdout().flush().unwrap();

        let builtin = vec!["type", "echo", "exit", "pwd", "cd"];
        let mut command = String :: new();

        io::stdin()
            .read_line(&mut command)
            .expect("Unable to read the input");

        let trimmed = command.trim();
        let parts = parse_args(trimmed);           
        if parts.is_empty() { continue; }                  
        let first_word_str = parts[0].as_str();             
        let args: Vec<&str> = parts[1..].iter().map(|s| s.as_str()).collect(); 

        if command.trim() == "exit"{
            break;
        }else if first_word_str == "echo" {
            println!("{}", args.join(" "));
            continue;
        }else if first_word_str == "type"{
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

        }else if first_word_str == "pwd" {
            if let Ok(cwd) = env::current_dir() {
                println!("{}", cwd.display());
            }
            continue;
        }else if first_word_str == "cd" {
            if args.is_empty() || args[0] =="~" {
                if let Ok(home) = env::var("HOME") {
                    change_directory(&home);
            }
            } else if !change_directory(args[0]) {
                println!("cd: {}: No such file or directory", args[0]);
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

    fn change_directory(path: &str) -> bool {
        match env::set_current_dir(path) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn parse_args(line: &str) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        let mut current = String::new();
        let mut in_single = false;
        let mut in_double = false;
        let mut escape = false;

        let mut chars = line.chars().peekable();

        while let Some(ch) = chars.next() {
            if escape {
                current.push(ch);
                escape = false;
                continue;
            }

            if in_double {
                match ch {
                    '\\' => {
                        if let Some(&next) = chars.peek() {
                            if next == '"' || next == '\\' {
                                let escaped = chars.next().unwrap();
                                current.push(escaped);
                            } else {
                                current.push('\\');
                            }
                        } else {
                            current.push('\\');
                        }
                    }
                    '"' => {
                        in_double = false;
                    }
                    _ => {
                        current.push(ch);
                    }
                }
            } else if in_single {
                match ch {
                    '\\' => {
                        if let Some(&next) = chars.peek() {
                            if next == '\\' {
                                chars.next();           
                                current.push('\\');    
                            } else {
                                current.push('\\');     
                            }
                        } else {
                            current.push('\\');
                        }
                    }
                    '\'' => {
                        in_single = false;
                    }
                    _ => {
                        current.push(ch);
                    }
                }
            } else {
                match ch {
                    '\\' => {
                        escape = true;
                    }
                    '"' => {
                        in_double = true;
                    }
                    '\'' => {
                        in_single = true;
                    }
                    ' ' | '\t' => {
                        if !current.is_empty() {
                            args.push(current.clone());
                            current.clear();
                        }
                    }
                    _ => {
                        current.push(ch);
                    }
                }
            }
        }

        if !current.is_empty() {
            args.push(current);
        }

        args
    }
}
