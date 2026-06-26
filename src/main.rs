use std::path::{Path, PathBuf};

use clap::Parser;
use image::{DynamicImage, GenericImage, ImageReader, Rgba, RgbaImage, imageops::FilterType};
use rayon::prelude::*;

/// Preprocesses images for makeplayingcards.com.
///
/// Steps:
/// 1. Replaces all transparent pixels with opaque black.
/// 2. Resizes the image using nearest-neighbor filtering to the specified width and height.
/// 3. Adds a margin of black pixels.
#[derive(Parser)]
#[clap(verbatim_doc_comment)]
struct Args {
    /// Width (pixels) for nearest-neighbor rescaling BEFORE adding margin
    #[arg(short = 'W', long, default_value_t = 744)]
    width: u32,

    /// Height (pixels) for nearest-neighbor rescaling BEFORE adding margin
    #[arg(short = 'H', long, default_value_t = 1038)]
    height: u32,

    /// Margin (pixels) to add to each side AFTER rescaling. Each dimension will be
    /// increased by 2x the margin. The added pixels are black.
    #[arg(short = 'M', long, default_value_t = 36)]
    margin: u32,

    /// Input directory of images.
    input_dir: PathBuf,

    /// Output directory of images. Does not need to exist beforehand. WILL BE OVERWRITTEN.
    output_dir: PathBuf,
}

fn load(path: impl AsRef<Path>) -> Result<DynamicImage, Box<dyn std::error::Error>> {
    Ok(ImageReader::open(path)?.decode()?)
}

fn flatten_transparent(img: &mut RgbaImage) {
    for px in img.pixels_mut() {
        if px[3] < 255 {
            *px = Rgba([0, 0, 0, 255]);
        }
    }
}

fn add_margin(img: &RgbaImage, margin: u32) -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let (w, h) = img.dimensions();
    let mut out = RgbaImage::from_pixel(w + 2 * margin, h + 2 * margin, Rgba([0, 0, 0, 255]));
    out.copy_from(img, margin, margin)?;
    Ok(out)
}

fn resize(img: &RgbaImage, width: u32, height: u32) -> RgbaImage {
    image::imageops::resize(img, width, height, FilterType::Nearest)
}

fn save(img: &RgbaImage, path: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
    img.save(path)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    std::fs::create_dir_all(&args.output_dir)?;

    let entries = std::fs::read_dir(&args.input_dir)?.collect::<Vec<_>>();
    entries.into_par_iter().for_each(|entry| {
        let Ok(entry) = entry else {
            println!("Not a dir entry");
            return;
        };
        let path = entry.path();
        if !path.is_file() {
            println!("Not a file");
            return;
        }
        let Some(name) = path.file_name() else {
            println!("No file name found");
            return;
        };
        let out_path = args.output_dir.join(name);
        println!("{} -> {}", path.display(), out_path.display());

        let mut img = load(&path).expect("could not load file").to_rgba8();
        flatten_transparent(&mut img);
        let img = resize(&img, args.width, args.height);
        let img = add_margin(&img, args.margin).expect("could not add margin");
        save(&img, &out_path).expect("could not load file");
    });
    Ok(())
}
