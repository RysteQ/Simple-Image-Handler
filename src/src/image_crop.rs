use image::{open, GenericImageView};

pub fn crop_image(input_image_location: String, output_image_location: String, start_x: u32, start_y: u32, end_x: u32, end_y: u32) {
    let mut image_to_crop = open(input_image_location).unwrap();

    if image_to_crop.dimensions().0 < end_x || image_to_crop.dimensions().1 < end_y {
        println!("Crop selection is bigger than the actuan image");
        std::process::exit(-1)
    }

    image_to_crop = image_to_crop.crop(start_x, start_y, end_x - start_x, end_y - start_y);
    image_to_crop.save(output_image_location).expect("Error saving image");
}