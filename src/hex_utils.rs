use eframe::egui::ColorImage;
use faster_hex::hex_decode;

pub fn string_to_hex(str: String) -> Vec<u8> {
    let href: &[u8]= str.as_ref();
    let mut dst = vec![0; href.len() / 2];
    hex_decode(href, &mut dst).unwrap();
    dst
}

pub fn load_image_from_memory(image_data: &[u8]) -> Result<ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels: image::FlatSamples<&[u8]> = image_buffer.as_flat_samples();
    Ok(ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}