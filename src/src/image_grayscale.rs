use image::open;

pub fn grayscale_image(image_file_location: String, save_location: String) {
    let grayscale_image = open(image_file_location).unwrap().grayscale();
    grayscale_image.save(save_location).unwrap();
}