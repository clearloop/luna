use std::env;
use std::{io, io::Write};
    
#[allow(dead_code)]
pub struct CLI { commands: Vec<Command> }

impl CLI {
    /// TODO: termion
    pub fn console() {
        loop {
            print!("$luna: ");
            let mut val = String::new();
            
            io::stdout().flush().unwrap();            
            io::stdin().read_line(&mut val).unwrap();

            match val.as_str() {
                "exit\n" | "quit\n" => break,
                _ => print!("your input is: {}", val)
            }
        }
    }
    
    pub fn exec() {
        match env::args().len() {
            1 => CLI::help(), _ => ()
        }
    }    
    
    fn help() {
        println!("
NAME:
    Luna - Micro smart-contract platform.\n
USAGE:
    luna <command> <options> <arguments...>\n
COMMANDS:
    account - Manage account\n
VERSION: Nightly\n 
Copyright 2019 clearloop <udtrokia@gmail.com>
");
    }
}

#[allow(dead_code)]
pub struct Command { sub_command: Vec<Command> }
pub struct Option { flag: &'static str, exec: fn() -> () }

trait Config {
    fn helper(s: &'static str) -> Self;
    fn option(s: &'static str) -> Self;
}
