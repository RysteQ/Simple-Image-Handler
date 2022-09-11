use image::GenericImage;
use image::GenericImageView;
use image::Pixel;
use image::open;
use image::Rgba;

pub fn single_out_colour(input_image_location: String, output_image_location: String, colour_vector: Vec<u8>) {
    let mut input_image = open(input_image_location).unwrap();

    for x in 0..input_image.dimensions().0 {
        for y in 0..input_image.dimensions().1 {
            let current_pixel: Rgba<u8> = input_image.get_pixel(x, y);
            let changed_pixel = Rgba([
                current_pixel.to_rgba()[0] * colour_vector[0],
                current_pixel.to_rgba()[1] * colour_vector[1],
                current_pixel.to_rgba()[2] * colour_vector[2],
                current_pixel.to_rgba()[3]
            ]);

            input_image.put_pixel(x, y, changed_pixel);
        }
    }

    input_image.save(output_image_location).unwrap();
}