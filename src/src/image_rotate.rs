use image::open;

pub fn rotate_image(input_image_location: String, output_image_location: String, degrees: String) {
    match degrees.as_str() {
        "90" => {
            let image_to_rotate = open(input_image_location).unwrap().rotate90();
            image_to_rotate.save(output_image_location).expect("Error saving image");
        },

        "180" => {
            let image_to_rotate = open(input_image_location).unwrap().rotate180();
            image_to_rotate.save(output_image_location).expect("Error saving image");
        },

        "270" => {
            let image_to_rotate = open(input_image_location).unwrap().rotate270();
            image_to_rotate.save(output_image_location).expect("Error saving image");
        },

        _ => {
            println!("Invalid argument {degrees}, acceptable arguments are 90 / 180 / 270");
            std::process::exit(-1);
        }
    }
}