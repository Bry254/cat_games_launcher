use egui::ColorImage;
use image::ImageReader;

pub fn load_image(
    ctx: &egui::Context,
    path: &String,
    name: &String,
) -> anyhow::Result<egui::TextureHandle> {
    let img = image::open(path)?.to_rgba8();
    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba);
    return Ok(ctx.load_texture(name, color_image, egui::TextureOptions::default()));
}

pub fn load_embedded_texture(
    ctx: &eframe::CreationContext<'_>,
    texture_name: &str,
    image: &[u8],
) -> anyhow::Result<egui::TextureHandle> {
    let img = ImageReader::new(std::io::Cursor::new(image))
        .with_guessed_format()?
        .decode()?;

    let size = [img.width() as usize, img.height() as usize];
    let rgba = img.to_rgba8().into_raw();
    let color_image = ColorImage::from_rgba_unmultiplied(size, &rgba);
    return Ok(ctx.egui_ctx.load_texture(
        texture_name,
        color_image,
        egui::TextureOptions::default(),
    ));
}