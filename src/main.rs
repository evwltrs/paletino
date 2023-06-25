use std::collections::HashMap;

fn main() {
    // Open the image file
    let img = image::open("image.png").expect("Failed to open image");

    // Resize the image to a smaller size for faster processing
    let resized_img = img.resize(100, 100, image::imageops::FilterType::Lanczos3);

    // Convert the image to the RGB color space
    let rgb_img = resized_img.into_rgb8();

    // Count the occurrences of each color
    let color_counts = count_colors(&rgb_img);

    // Sort the colors by their count in descending order
    let sorted_colors = sort_colors(color_counts);

    // Get the top 10 most common colors
    let top_10_colors = sorted_colors.into_iter().take(10).collect::<Vec<_>>();

    // Print the top 10 colors
    for (color, count) in top_10_colors {
        println!("Color: {:?}, Count: {}", color, count);
    }
}

fn count_colors(image: &image::RgbImage) -> HashMap<image::Rgb<u8>, u32> {
    let mut color_counts: HashMap<image::Rgb<u8>, u32> = HashMap::new();

    for (_, _, pixel) in image.enumerate_pixels() {
        let count = color_counts.entry(*pixel).or_insert(0);
        *count += 1;
    }

    color_counts
}

fn sort_colors(color_counts: HashMap<image::Rgb<u8>, u32>) -> Vec<(image::Rgb<u8>, u32)> {
    let mut sorted_colors: Vec<(image::Rgb<u8>, u32)> = color_counts.into_iter().collect();
    sorted_colors.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by count in descending order
    sorted_colors
}
