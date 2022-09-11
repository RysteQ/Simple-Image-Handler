use std::env::args;
use std::vec;

mod analyze_command;

mod help;

mod image_grayscale;
mod image_resize;
mod image_convert_to_ascii;
mod image_single_out_colour;
mod image_blur;

use crate::help::show_help;

use crate::analyze_command::CommandStruct;
use crate::analyze_command::analyze_command;
use crate::analyze_command::ProcessTypeEnum;

use crate::image_convert_to_ascii::convert_image_to_ascii;
use crate::image_grayscale::grayscale_image;
use crate::image_resize::resize_image;
use crate::image_single_out_colour::single_out_colour;
use crate::image_blur::blur;

fn main() {
    // let command_arguments: Vec<String> = args().collect();
    
    // DEBUG COMMAND
    let command_arguments: Vec<String> = vec!["simple_image_handler".to_string(), "/home/rysteq/Downloads/birb.jpg".to_string(), "/home/rysteq/Downloads/birb_out.jpg".to_string(), "-blur".to_string(), "1.3".to_string()];

    if command_arguments.len() < 4 {
        show_help();
    }

    let command: CommandStruct = analyze_command(command_arguments);

    match command.process_type {
        ProcessTypeEnum::Grayscale => {
            if check_arguments_validity(command.extra_parameters, vec![], 0) == false {
                handle_error(-1, "Invalid extra parameters, there should be no extra arguments");
            }

            grayscale_image(command.input_file, command.output_file)
        },

        ProcessTypeEnum::Resize => {
            if command.extra_parameters.len() != 2 {
                handle_error(-1, "Invalid extra parameters");
            }

            let width: u32 =  command.extra_parameters[0].clone().parse::<u32>().expect("Error converting string to u32");
            let height: u32 = command.extra_parameters[1].clone().parse::<u32>().expect("Error converting string to u32");

            resize_image(command.input_file, command.output_file, width, height);            
        },

        ProcessTypeEnum::ConvertToAsciiCharacters => {
            if check_arguments_validity(command.extra_parameters, vec![], 0) == false {
                handle_error(-1, "Invalid extra parameters, there should be no extra arguments");
            }

            convert_image_to_ascii(command.input_file, command.output_file)
        },

        ProcessTypeEnum::SingleOutColour => {
            if check_arguments_validity(command.extra_parameters.clone(), vec!["-red".to_string(), "-green".to_string(), "-blue".to_string()], 1) == false {
                handle_error(-1, "Invalid extra parameters, there should be one extra arguments");
            }

            let mut colour_multiplier_vector: Vec<u8> = vec![0, 0, 0];

            match command.extra_parameters[0].as_str() {
                "-red" => colour_multiplier_vector = vec![1, 0, 0],
                "-green" => colour_multiplier_vector = vec![0, 1, 0],
                "-blue" => colour_multiplier_vector = vec![0, 0, 1],
                _ => { }
            }

            single_out_colour(command.input_file, command.output_file, colour_multiplier_vector);
        }

        ProcessTypeEnum::Blur => {
            if command.extra_parameters.len() != 1  {
                handle_error(-1, "Invalid extra parameters, there should be no extra arguments");
            }

            let sigma: f32 = command.extra_parameters[0].clone().parse::<f32>().expect("Error converting string to float");

            blur(command.input_file, command.output_file, sigma);
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

// TODO: maybe add a description parameter for how a command should be structured ?
fn handle_error(error_code: i32, error_msg: &str) {
    println!("{error_msg}");
    std::process::exit(error_code);
}