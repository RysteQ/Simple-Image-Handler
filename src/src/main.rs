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
            if check_arguments_validity(command.extra_parameters, vec![], 0) == false {
                handle_error(-1, "Invalid extra parameters, there should be no extra arguments");
            }

            // TODO: new file.rs for grayscale convertion
        },

        ProcessTypeEnum::Resize => {
            if command.extra_parameters.len() != 2 || valid_nums(command.extra_parameters) == false {
                handle_error(-1, "Invalid extra parameters");
            }

            // TODO: new file.rs of image resize
        },

        ProcessTypeEnum::ConvertToAsciiCharacters => {
            if check_arguments_validity(command.extra_parameters, vec![], 0) == false {
                handle_error(-1, "Invalid extra parameters, there should be no extra arguments");
            }

            // TODO: new file.rs for ASCII convertion
        },

        ProcessTypeEnum::SingleOutColour => {
            if check_arguments_validity(command.extra_parameters, vec!["-red".to_string(), "-green".to_string(), "-blue".to_string()], 0) == false {
                handle_error(-1, "Invalid extra parameters, there should be no extra arguments");
            }

            // TODO: new file.rs to single out a certain colour in an image
        }
    }

    // TODO: print out the ms it took to finish the operation
    println!("Done !");
}

fn check_arguments_validity(arguments_vector: Vec<String>, accepted_arguments: Vec<String>, accepted_length: usize) -> bool {
    if arguments_vector.len() != accepted_length {
        return false;
    }

    if accepted_length == 0 {
        return true;
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

fn valid_nums(numbers_vector: Vec<String>) -> bool {
    for i in 0..numbers_vector.len() {
        match numbers_vector[i].parse::<u32>() {
            Ok(ok) => { },
            Err(err) => {
                return false;
            }
        }
    }

    return true;
}

// TODO: maybe add a description parameter for how a command should be structured ?
fn handle_error(error_code: i32, error_msg: &str) {
    println!("{error_msg}");
    std::process::exit(error_code);
}