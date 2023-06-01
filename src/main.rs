use std::io::{self, Write};
use pyu_rust_util as pyu;

fn main() {
    let cmds = ["exit", "help", "curl", "shell"];
    println!("Welcome to: ");
    println!("
██████  ██    ██ ███████ ████████       ████████  █████  ███████ ██   ██ 
██   ██ ██    ██ ██         ██             ██    ██   ██ ██      ██  ██  
██████  ██    ██ ███████    ██    █████    ██    ███████ ███████ █████   
██   ██ ██    ██      ██    ██             ██    ██   ██      ██ ██  ██  
██   ██  ██████  ███████    ██             ██    ██   ██ ███████ ██   ██ 



    ");

    println!("Type 'help' for a list of commands.");
    loop {
        let string = pyu::input("# ").to_owned().to_lowercase();
        let cmd = string.trim(); 

        match cmd {
            "exit" => {
                return;
            },

            "help" => {
                println!("Commands:\n");
                
                for v in cmds {
                    println!("{}", v)
                }                
            }

            "curl" => {
                let url = pyu::input("Enter an url: ");
                
                pyu::curl(&url.trim())
            }

            "shell" => {
                println!("Use basic shell commands.");
                let input = pyu::input("Enter shell command: ").to_owned().to_lowercase();
                let shell = input.trim();

                match shell {
                    "ls" => {
                        let path = pyu::input("Enter path: ");

                        io::stdout().write_all(&pyu::exec("ls", path.trim()).stdout).expect("Could not write stdout.");
                    }

                    "mkdir" => {
                        let path = pyu::input("Enter path: ");

                        io::stdout().write_all(&pyu::exec("mkdir", path.trim()).stdout).expect("Could not write stdout.");
                    }

                    _ => {
                        let cmd = pyu::input("Enter command: ");
                        let arg = pyu::input("Enter argument: ");
                        io::stdout().write_all(&pyu::exec(cmd.trim(), arg.trim()).stdout).expect("Could not write stdout."); 
                    }
                }
            }

            _ => {
                println!("Invalid command");
            }
        }
    }
}
