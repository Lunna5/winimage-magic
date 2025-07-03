use std::io::Cursor;

pub fn read_image_from_path(path: &str)
                            -> Result<image::DynamicImage, image::ImageError> {
    image::open(path)
}

pub fn convert_image_format(
    image: &image::DynamicImage,
    format: image::ImageFormat,
) -> Result<Cursor<Vec<u8>>, image::ImageError> {
    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    image.write_to(&mut buffer, format)?;
    Ok(buffer)
}

pub fn write_image_to_path(
    buffer: &[u8],
    path: &str
) -> Result<(), image::ImageError> {
    let mut file = std::fs::File::create(path)?;
    std::io::Write::write_all(&mut file, buffer)?;
    Ok(())
}

pub fn convert_image_to_format_and_write(
    input_path: &str,
    output_path: &str,
    format: image::ImageFormat,
) -> Result<(), image::ImageError> {
    let image = read_image_from_path(input_path)?;
    let buffer = convert_image_format(&image, format)?;
    write_image_to_path(buffer.get_ref(), output_path)?;
    Ok(())
}
