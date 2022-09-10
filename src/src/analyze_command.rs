const RESIZE_COMMAND: &str = "-rs";
const GRAYSCALE_COMMAND: &str = "-gs";
const SINGLE_OUT_COLOUR_COMMAND: &str = "-soc";
const CONVERT_TO_ASCII_CHARACTER_COMMAND: &str = "-ctac";

pub enum ProcessTypeEnum {
    Resize,
    Grayscale,
    SingleOutColour,
    ConvertToAsciiCharacters
}

pub struct command_struct {
    pub process_type: ProcessTypeEnum,
    pub input_file: String,
    pub output_file: String,
    pub extra_parameters: Vec<String>
}

pub fn analyze_command(command_arguments_vector: Vec<String>) -> command_struct {
    let mut to_return: command_struct = command_struct { 
        process_type: ProcessTypeEnum::Grayscale, 
        input_file: String::new(), 
        output_file: String::new(), 
        extra_parameters: vec![]
    };

    match command_arguments_vector[3].as_str() {
        RESIZE_COMMAND => to_return.process_type = ProcessTypeEnum::Resize,
        GRAYSCALE_COMMAND => to_return.process_type = ProcessTypeEnum::Grayscale,
        SINGLE_OUT_COLOUR_COMMAND => to_return.process_type = ProcessTypeEnum::SingleOutColour,
        CONVERT_TO_ASCII_CHARACTER_COMMAND => to_return.process_type = ProcessTypeEnum::ConvertToAsciiCharacters,

        _ => { 
            println!("Error analyzing command, unknown operation {}", command_arguments_vector[3]);
            std::process::exit(-1);
        }
    }

    to_return.input_file = command_arguments_vector[1].to_string();
    to_return.output_file = command_arguments_vector[2].to_string();

    if command_arguments_vector.len() > 4 {
        to_return.extra_parameters = command_arguments_vector[4..].to_vec();
    }

    return to_return;
}