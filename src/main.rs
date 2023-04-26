use std::path::Path;

use anyhow::*;
use image::{io::Reader as ImageReader, ImageFormat, Rgba};

fn main() -> Result<()> {
    let lut_intensity = 100.0;

    let path = Path::new(r"WANT_TO_APPLY_LUT_TO_IMAGE'S_PATH");
    let file_name = path.file_stem().unwrap().to_string_lossy() + "_applied";
    let file_etx = path.extension().unwrap();
    let mut new_path = path.to_owned();

    let source = ImageReader::open(&path)?.decode()?.into_rgba8();
    let mut source_mut = source.clone();

    let lut_img = ImageReader::open(r"GSHADE_LUT_PATH")?.decode()?;
    let lut_width = lut_img.width();
    let lut_vec = lut_img.clone().into_rgba8().to_vec();

    let lut_tile_xy: usize = 32;
    let lut_tile_amount: usize = 32;
    let lut_selected: usize = 5;
    let coeff: usize = (255 as f32 / lut_tile_xy as f32).round() as usize;

    for (x, y, pixel_rgba) in source.enumerate_pixels() {
        let (pr, pg, pb, pa) = (
            pixel_rgba[0] as usize,
            pixel_rgba[1] as usize,
            pixel_rgba[2] as usize,
            pixel_rgba[3],
        );
        let (r, g, b) = (pr / coeff, pg / coeff, pb / coeff);
        let lut_x = (b % lut_tile_xy) * lut_tile_xy + r;
        let lut_y = (b / lut_tile_amount) * lut_tile_amount + g;
        let lut_index = ((lut_y as u32 + (lut_selected as u32 * lut_tile_xy as u32)) * lut_width
            + lut_x as u32)
            * 4;
        let lut_r = lut_vec[lut_index as usize];
        let lut_g = lut_vec[lut_index as usize + 1];
        let lut_b = lut_vec[lut_index as usize + 2];

        let (r, g, b) = (
            lerp(pr, lut_r.into(), lut_intensity / 100.0),
            lerp(pg, lut_g.into(), lut_intensity / 100.0),
            lerp(pb, lut_b.into(), lut_intensity / 100.0),
        );

        source_mut.put_pixel(x, y, Rgba([r as u8, g as u8, b as u8, pa]));
    }

    new_path.set_file_name(file_name.to_string() + "." + file_etx.to_str().unwrap_or("png"));
    source_mut.save_with_format(&new_path, ImageFormat::from_extension(file_etx).unwrap())?;

    Ok(())
}

fn lerp(a: usize, b: usize, n: f32) -> usize {
    ((1.0 - n) * a as f32 + n * b as f32) as usize
}
