use paletino::median_cut;

fn main() {
    let image = image::open("image.png").unwrap();
    let num_colors = 8; // Number of colors in the palette

    let palette = median_cut(&image, num_colors);
    for color in palette {
        // println!("{}, {}, {}", color.r, color.g, color.b);
        println!("{:?}", color);
    }
}
