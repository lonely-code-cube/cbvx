use color_thief::{get_palette, ColorFormat};
use image::{io::Reader as Imgreader, GenericImage, GenericImageView};
use pyo3::prelude::*;

#[pyclass]
pub struct Color {
    pub r: u8,
    pub b: u8,
    pub g: u8,
}

impl Color {
    fn new(r: u8, b: u8, g: u8) -> Self {
        Self { r, b, g }
    }
}

#[pyfunction]
pub fn pallet(pixels: &[u8]) -> PyResult<(Color, Color)> {
    let pallet =
        get_palette(pixels, ColorFormat::Rgb, 1, 2).expect("Couldnt generate pallet from pixels");
    let (c1, c2) = (pallet[0], pallet[1]);
    Ok((Color::new(c1.r, c1.b, c1.g), Color::new(c2.r, c2.b, c2.g)))
}
