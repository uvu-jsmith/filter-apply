use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, GrayImage, RgbaImage};

#[derive(Parser, Debug)]
#[command(author="Jess Smith", version, about="Simple image color filter program", long_about = None)]
struct Args {
    ifile: String,
    r: u8,
    g: u8,
    b: u8,
    intensity: f64,
}
struct ImageFilter {
    image: DynamicImage,
    r: u8,
    g: u8,
    b: u8,
    intensity: f64,
}

/// Legacy code (commentted out so superior code is used)
// fn apply_sepia(image: DynamicImage) -> DynamicImage {
//     let (width, height) = image.dimensions();
//     let mut sepia_image = ImageBuffer::new(width, height);

//     for y in 0..height {
//         for x in 0..width {
//             let pixel = image.get_pixel(x, y);

//             let rgba = pixel.0;
//             let r = rgba[0];
//             let g = rgba[1];
//             let b = rgba[2];
//             let a = rgba[3];

//             let new_r = (0.393 * r as f64 + 0.769 * g as f64 + 0.189 * b as f64).min(255.0) as u8;
//             let new_g = (0.349 * r as f64 + 0.686 * g as f64 + 0.168 * b as f64).min(255.0) as u8;
//             let new_b = (0.272 * r as f64 + 0.534 * g as f64 + 0.131 * b as f64).min(255.0) as u8;

//             let sepia_pixel = Rgba([new_r, new_g, new_b, a]);
//             sepia_image.put_pixel(x, y, sepia_pixel);
//         }
//     }

//     DynamicImage::ImageRgba8(sepia_image)
// }

// ChatGPT-inspired function (needed to make grayscale images RGB-manipulatable)
fn convert_to_dynamic(input: &GrayImage) -> DynamicImage {
    // Create a new RgbaImage to store the RGB values from the GrayImage
    let mut rgba_image = RgbaImage::new(input.width(), input.height());

    // Iterate over each pixel in the GrayImage and set the corresponding RGB value in the RgbaImage
    for (x, y, pixel) in rgba_image.enumerate_pixels_mut() {
        let luminance = input.get_pixel(x, y).0[0];
        *pixel = Rgba([luminance, luminance, luminance, 255]);
    }

    // Convert the RgbaImage to a DynamicImage
    DynamicImage::ImageRgba8(rgba_image)
}

fn apply_filter(pre: ImageFilter) -> DynamicImage {
    let (width, height) = pre.image.dimensions();
    let f_r = pre.r as f64;
    let f_g = pre.g as f64;
    let f_b = pre.b as f64;
    let f_intensity = pre.intensity;
    let gray = pre.image.to_luma8();
    let gray_conv = convert_to_dynamic(&gray);
    let mut filtered_image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = gray_conv.get_pixel(x, y);

            let rgba = pixel.0;
            let r = rgba[0];
            let g = rgba[1];
            let b = rgba[2];
            let a = rgba[3];

            let new_r = ((f_r/(255.0)) * r as f64 * f_intensity).min(255.0) as u8;
            let new_g = ((f_g/(255.0)) * g as f64 * f_intensity).min(255.0) as u8;
            let new_b = ((f_b/(255.0)) * b as f64 * f_intensity).min(255.0) as u8;

            let filtered_pixel = Rgba([new_r, new_g, new_b, a]);
            filtered_image.put_pixel(x, y, filtered_pixel);
        }
    }
    DynamicImage::ImageRgba8(filtered_image)
}
fn main() {
    let args = Args::parse();

    println!("Opening {}", &args.ifile);
    let input_image = image::open(&args.ifile).expect("Failed to open the input file");
    let pre_filter = ImageFilter{image:input_image, r:args.r, g:args.g, b:args.b, intensity:args.intensity};
    let output_image = apply_filter(pre_filter);

    let output_filename = format!("{}-filtered.jpg", &args.ifile);
    output_image.save(output_filename).expect("Failed to save the output file");
}