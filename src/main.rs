use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ifile: String,
}

fn apply_sepia(image: DynamicImage) -> DynamicImage {
    let (width, height) = image.dimensions();
    let mut sepia_image = ImageBuffer::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            let rgba = pixel.0;
            let r = rgba[0];
            let g = rgba[1];
            let b = rgba[2];
            let a = rgba[3];

            let new_r = (0.393 * r as f64 + 0.769 * g as f64 + 0.189 * b as f64).min(255.0) as u8;
            let new_g = (0.349 * r as f64 + 0.686 * g as f64 + 0.168 * b as f64).min(255.0) as u8;
            let new_b = (0.272 * r as f64 + 0.534 * g as f64 + 0.131 * b as f64).min(255.0) as u8;

            let sepia_pixel = Rgba([new_r, new_g, new_b, a]);
            sepia_image.put_pixel(x, y, sepia_pixel);
        }
    }

    DynamicImage::ImageRgba8(sepia_image)
}


fn main() {
    let args = Args::parse();

    println!("Opening {}", &args.ifile);
    let input_image = image::open(&args.ifile).expect("Failed to open the input file");

    let output_image = apply_sepia(input_image);

    let output_filename = format!("sepia_{}", &args.ifile);
    output_image.save(output_filename).expect("Failed to save the output file");
}