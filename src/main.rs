use image::{DynamicImage, ImageBuffer};

fn main() {
    // Load an image from file
    let img = image::open("input.jpg").expect("Failed to open image");

    // Apply a grayscale filter
    let gray_img = grayscale(&img);

    // Save the result
    gray_img.save("output.jpg").expect("Failed to save image");
}

fn grayscale(image: &DynamicImage) -> ImageBuffer<image::Luma<u8>, Vec<u8>> {
    image.to_luma8()
}
