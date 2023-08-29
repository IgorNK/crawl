use eframe::egui;
use std::path::PathBuf;
use std::sync::Arc;
use bytes::bytes::Bytes;

pub fn load_image_from_bytes(bytes: Arc<Bytes>) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(&bytes)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

pub fn load_image_from_path(path: &PathBuf) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::io::Reader::open(path)?.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}
