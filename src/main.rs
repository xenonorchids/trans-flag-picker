use image::ImageBuffer;
use num_integer::Roots;
use std::env;

const PINK: image::Rgb<u8> = image::Rgb([247, 168, 184]);
const WHITE: image::Rgb<u8> = image::Rgb([255, 255, 255]);
const BLUE: image::Rgb<u8> = image::Rgb([85, 205, 252]);

fn main() {
    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 2, "Missing arguments");
    

    let im = image::open(&args[1]).unwrap().to_rgb8();
    let pixels_rgb: Vec<u8> = im.into_raw();

    let result_h: usize = 400;
    let result_w: usize = 700;

    let bar_size: usize = result_h * result_w / 5;

    let mut closest_val_pink: i32 = i32::MAX;
    let mut closest_val_white: i32 = i32::MAX;
    let mut closest_val_blue: i32 = i32::MAX;

    let mut closest_px_pink: Vec<u8> = vec![0, 0, 0];
    let mut closest_px_white: Vec<u8> = vec![0, 0, 0];
    let mut closest_px_blue: Vec<u8> = vec![0, 0, 0];

    let mut result_img: Vec<u8> = vec![0; (result_h * result_w * 3) as usize];

    for i in 0..pixels_rgb.len() / 3 {
        let dist_pink = get_distance(
            pixels_rgb[i * 3],
            pixels_rgb[(i * 3) + 1],
            pixels_rgb[(i * 3) + 2],
            PINK[0],
            PINK[1],
            PINK[2],
        );
        let dist_white = get_distance(
            pixels_rgb[i * 3],
            pixels_rgb[(i * 3) + 1],
            pixels_rgb[(i * 3) + 2],
            WHITE[0],
            WHITE[1],
            WHITE[2],
        );
        let dist_blue = get_distance(
            pixels_rgb[i * 3],
            pixels_rgb[(i * 3) + 1],
            pixels_rgb[(i * 3) + 2],
            BLUE[0],
            BLUE[1],
            BLUE[2],
        );

        if dist_pink < closest_val_pink {
            closest_val_pink = dist_pink;
            closest_px_pink = vec![
                pixels_rgb[i * 3],
                pixels_rgb[(i * 3) + 1],
                pixels_rgb[(i * 3) + 2],
            ];
        }

        if dist_white < closest_val_white {
            closest_val_white = dist_white;
            closest_px_white = vec![
                pixels_rgb[i * 3],
                pixels_rgb[(i * 3) + 1],
                pixels_rgb[(i * 3) + 2],
            ];
        }

        if dist_blue < closest_val_blue {
            closest_val_blue = dist_blue;
            closest_px_blue = vec![
                pixels_rgb[i * 3],
                pixels_rgb[(i * 3) + 1],
                pixels_rgb[(i * 3) + 2],
            ];
        }
    }

    for row in 0..5 {
        match row {
            0 => {
                // blue stripe
                for i in 0..bar_size {
                    result_img[i * 3] = closest_px_blue[0];
                    result_img[(i * 3) + 1] = closest_px_blue[1];
                    result_img[(i * 3) + 2] = closest_px_blue[2];
                }
            }
            1 => {
                // pink stripe
                for i in bar_size..bar_size * 2 {
                    result_img[i * 3] = closest_px_pink[0];
                    result_img[(i * 3) + 1] = closest_px_pink[1];
                    result_img[(i * 3) + 2] = closest_px_pink[2];
                }
            }
            2 => {
                // white stripe
                for i in bar_size * 2..bar_size * 3 {
                    result_img[i * 3] = closest_px_white[0];
                    result_img[(i * 3) + 1] = closest_px_white[1];
                    result_img[(i * 3) + 2] = closest_px_white[2];
                }
            }
            3 => {
                // pink stripe
                for i in bar_size * 3..bar_size * 4 {
                    result_img[i * 3] = closest_px_pink[0];
                    result_img[(i * 3) + 1] = closest_px_pink[1];
                    result_img[(i * 3) + 2] = closest_px_pink[2];
                }
            }
            4 => {
                // blue stripe
                for i in bar_size * 4..bar_size * 5 {
                    result_img[i * 3] = closest_px_blue[0];
                    result_img[(i * 3) + 1] = closest_px_blue[1];
                    result_img[(i * 3) + 2] = closest_px_blue[2];
                }
            }
            _ => {}
        }
    }

    let new_image: ImageBuffer<image::Rgb<u8>, _> =
        image::ImageBuffer::from_raw(result_w as u32, result_h as u32, result_img).unwrap();
    new_image.save("output.png").unwrap();
    println!("Done!")
}

fn get_distance(r1: u8, g1: u8, b1: u8, r2: u8, g2: u8, b2: u8) -> i32 {
    let rgb1 = image::Rgb([r1 as i32, g1 as i32, b1 as i32]);
    let rgb2 = image::Rgb([r2 as i32, g2 as i32, b2 as i32]);

    let result: i32 = (i32::pow(rgb2[0] - rgb1[0], 2)
        + i32::pow(rgb2[1] - rgb1[1], 2)
        + i32::pow(rgb2[2] - rgb1[2], 2))
    .abs()
    .sqrt() as i32;
    result
}
