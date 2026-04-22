use image;

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    JPG,
    AVIF,
    BMP,
    GIF,
    PNG,
    QOI,
    TGA,
    TIFF,
    WEBP,
    None,
}

impl From<&String> for Format {
    fn from(s: &String) -> Self {
        match s.as_str() {
            "jpg" => Format::JPG,
            "avif" => Format::AVIF,
            "bmp" => Format::BMP,
            "gif" => Format::GIF,
            "png" => Format::PNG,
            "qoi" => Format::QOI,
            "tga" => Format::TGA,
            "tiff" => Format::TIFF,
            "webp" => Format::WEBP,
            _ => Format::None,
        }
    }
}

impl Format {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Format::JPG => "jpg",
            Format::AVIF => "avif",
            Format::BMP => "bmp",
            Format::GIF => "gif",
            Format::PNG => "png",
            Format::QOI => "qoi",
            Format::TGA => "tga",
            Format::TIFF => "tiff",
            Format::WEBP => "webp",
            Format::None => "",
        }
    }

    pub fn values() -> Vec<Format> {
        vec![
            Format::PNG,
            Format::JPG,
            Format::GIF,
            Format::WEBP,
            Format::AVIF,
            Format::BMP,
            Format::QOI,
            Format::TGA,
            Format::TIFF,
        ]
    }
}

pub fn is_image_extension(file_ext: &String) -> bool {
    let format = Format::from(file_ext);
    match format {
        Format::None => false,
        _ => true,
    }
}

pub fn convert_image(input_file: String, ext: &str) -> anyhow::Result<()> {
    // Get the name of the image file
    println!("Converting image");
    let new_file_path = input_file.rsplit_once(".").unwrap().0;
    let new_path = format!("{}.{}", new_file_path, ext);

    // Open the image file
    let img = image::open(&input_file)?.to_rgb8();

    // Save the image file with required extension
    img.save(new_path)?;

    Ok(())
}
