use std::env;

#[allow(dead_code)]
pub struct CLI {
    commands: Vec<Command>
}

impl CLI {
    pub fn run() {
        match env::args().len() {
            1 => CLI::help(),
            _ => ()
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
pub struct Command {
    sub_command: Vec<Command>,
    option: Vec<&'static str>,
    helper: &'static str
}

#[allow(dead_code)]
trait Config {
    fn exec() -> Self;
    fn help(s: &'static str) -> Self;
    fn option(s: &'static str) -> Self;
}

fn main() {
    CLI::run();
}
