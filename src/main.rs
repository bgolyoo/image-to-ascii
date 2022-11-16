use std::{borrow::Borrow, env, path::Path};

use image::{DynamicImage, GenericImageView, imageops::FilterType, ImageResult, Pixels};

fn open_image(path: &str) -> ImageResult<DynamicImage> {
    image::open(&Path::new(path))
}

fn resize_image(image: DynamicImage, scale: u32) -> DynamicImage {
    let resized_image = image.resize(
        image.width() / scale,
        image.height() / scale,
        FilterType::Nearest,
    );

    println!(
        "Original size {:?}   Reduced: {:?}",
        image.dimensions(),
        resized_image.dimensions()
    );

    resized_image
}

fn generate_ascii_art(pixels: Pixels<DynamicImage>, characters: &[&str]) -> String {
    let mut art = String::new();
    let mut last_y = 0;
    for pixel in pixels {
        if last_y != pixel.1 {
            art.push_str("\n");
            last_y = pixel.1;
        }

        let pixel_data = pixel.2;
        let brightness: f64 = ((pixel_data[0] as u64 + pixel_data[1] as u64 + pixel_data[2] as u64) / 3) as f64;
        let character_position = ((brightness / 255.0) * (characters.len() - 1) as f64).round() as usize;
        art.push_str(&characters[character_position])
    }
    art
}

fn main() {
    // cargo run /absolute-path/../test/images/a.jpeg long
    // 1. arg is path to image
    // 2. arg is type of character set: short/long/shade
    let args: Vec<String> = env::args().collect();
    let file_path_input = &args[1];
    let characters_input = &args[2];

    println!("{:?}", file_path_input);

    let image = match open_image(file_path_input) {
        Ok(image) => image,
        Err(e) => panic!("error {:?}", e)
    };

    let short_character_set = [" ", "'", ",", ".", ":", ";", "L", "O", "0", "#", "@"];
    let long_character_set = [
        " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?", "@", "A", "B", "C",
        "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
        "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_", "`", "a", "b", "c", "d", "e", "f", "g",
        "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y",
        "z", "{", "|", "}", "~",
    ];
    let shade_character_set = [" ", "░", "█", "▄", "▀"];


    let scale = 4;
    let resized_image = resize_image(image, scale);

    // let art = generate_ascii_art(resized_image.pixels(), &characters);
    let art = match characters_input.borrow() {
        "short" => generate_ascii_art(resized_image.pixels(), &short_character_set),
        "long" => generate_ascii_art(resized_image.pixels(), &long_character_set),
        "shade" => generate_ascii_art(resized_image.pixels(), &shade_character_set),
        _ => generate_ascii_art(resized_image.pixels(), &shade_character_set),
    };

    print!("{}", art)
}
