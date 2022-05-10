const PINK: image::Rgb<u8> = image::Rgb([247,168,184]);
const WHITE: image::Rgb<u8> = image::Rgb([255,255,255]);
const BLUE: image::Rgb<u8> = image::Rgb([85,205,252]);

fn main() {
    let im = image::open("image.png").unwrap().to_rgb8();
    let pixels_rgb: Vec<u8> = im.into_raw();

    let mut closest_val_pink: i32 = 1000;
    let mut closest_val_white: i32 = 1000;
    let mut closest_val_blue: i32 = 1000;

    let mut closest_px_pink: Vec<u8> = vec![0,0,0];
    let mut closest_px_white: Vec<u8> = vec![0,0,0];
    let mut closest_px_blue: Vec<u8> = vec![0,0,0];

    for i in 0..pixels_rgb.len()/3{
        let dist_pink = get_distance(pixels_rgb[i*3], pixels_rgb[(i*3)+1], pixels_rgb[(i*3)+2], PINK[0], PINK[1], PINK[2]);
        let dist_white = get_distance(pixels_rgb[i*3], pixels_rgb[(i*3)+1], pixels_rgb[(i*3)+2], WHITE[0], WHITE[1], WHITE[2]);
        let dist_blue = get_distance(pixels_rgb[i*3], pixels_rgb[(i*3)+1], pixels_rgb[(i*3)+2], BLUE[0], BLUE[1], BLUE[2]);

        if dist_pink < closest_val_pink {
            closest_val_pink = dist_pink;
            closest_px_pink = vec![pixels_rgb[i*3], pixels_rgb[(i*3)+1], pixels_rgb[(i*3)+2]];
        }
        if dist_white < closest_val_white {
            closest_val_white = dist_white;
            closest_px_white = vec![pixels_rgb[i*3], pixels_rgb[(i*3)+1], pixels_rgb[(i*3)+2]];
        }
        if dist_blue < closest_val_blue {
            closest_val_blue = dist_blue;
            closest_px_blue = vec![pixels_rgb[i*3], pixels_rgb[(i*3)+1], pixels_rgb[(i*3)+2]];
        }

    }
    println!("Pink:{:?} \nWhite: {:?} \nBlue: {:?}", closest_px_pink, closest_px_white, closest_px_blue);
}

fn get_distance(r1:u8,g1:u8,b1:u8,r2:u8,g2:u8,b2:u8) -> i32 {
    let rgb1 = image::Rgb([r1 as i16,g1 as i16,b1 as i16]);
    let rgb2 = image::Rgb([r2 as i16,g2 as i16,b2 as i16]);
    
    ((rgb1[0]-rgb2[0]) + (rgb1[1]-rgb2[1]) + (rgb1[2]-rgb2[2])).abs() as i32
}