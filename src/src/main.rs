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

    match command.process_type {
        ProcessTypeEnum::Grayscale => {

        },

        ProcessTypeEnum::Resize => {

        },

        ProcessTypeEnum::ConvertToAsciiCharacters => {

        },

        ProcessTypeEnum::SingleOutColour => {

        }
    }

    // TODO, print out the ms it took to finish the operation
    println!("Done !");
}

fn check_arguments_validity(arguments_vector: Vec<String>, accepted_arguments: Vec<String>, maximum_length: usize) -> bool {
    if arguments_vector.len() > maximum_length {
        return false;
    }

    for i in 0..arguments_vector.len() {
        for j in 0..accepted_arguments.len() {
            if arguments_vector[i] == accepted_arguments[j] {
                return true;
            }
        }
    }

    return false;
}