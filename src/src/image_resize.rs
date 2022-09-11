use image::open;
use image::imageops::FilterType;

pub fn resize_image(image_location: String, output_image_location: String, width: u32, height: u32) {
    let image_to_resize = open(image_location).unwrap().resize(width, height, FilterType::Nearest);
    image_to_resize.save(output_image_location).unwrap();
}