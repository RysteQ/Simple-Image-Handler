use image::{open, GenericImageView, Pixel};
use image::imageops::FilterType;
use std::fs;

pub fn convert_image_to_ascii(input_file_location: String, output_file_location: String) {
    let mut input_image = open(input_file_location).unwrap().grayscale();

    if input_image.dimensions().0 < 40 || input_image.dimensions().1 < 40 {
        println!("Image must be at least 40x40");
        std::process::exit(-1);
    }

    input_image = input_image.resize(40, 40, FilterType::Nearest);

    let intensity_map: [char; 5] = [
        ' ',
        '\"',
        'o',
        '@',
        '#'
    ];

    let mut character_output: Vec<char> = vec![];

    for x in 0..40 {
        for y in 0..40 {
            let rgb: usize = usize::from(input_image.get_pixel(x, y).to_rgb()[0]) / 60;
            character_output.push(intensity_map[rgb]);
        }

        character_output.push('\n');
    }

    let data_to_write: String = character_output.iter().collect();
    fs::write(output_file_location, data_to_write).expect("Error writing to output file");
}