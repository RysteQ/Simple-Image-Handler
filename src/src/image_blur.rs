use image::open;

pub fn blur(input_file_location: String, output_file_location: String, sigma: f32) {
    let blured_image = open(input_file_location).unwrap().blur(sigma);
    blured_image.save(output_file_location).unwrap();
}