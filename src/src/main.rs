use std::env::args;

mod analyze_command;

use crate::analyze_command::command_struct;
use crate::analyze_command::analyze_command;
use crate::analyze_command::ProcessTypeEnum;

fn main() {
    let command_arguments: Vec<String> = args().collect();

    if command_arguments.len() < 4 {
        println!("Invalid argument count");
        std::process::exit(-1);
    }

    let command: command_struct = analyze_command(command_arguments);
}
