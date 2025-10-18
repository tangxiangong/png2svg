use crate::{Error, Result};
use rayon::prelude::*;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};
use visioncortex::PathSimplifyMode;
use vtracer::{ColorImage, ColorMode, Config, Hierarchical};

pub fn convert_directory(
    dirpath: impl AsRef<Path>,
    output_dir: Option<impl AsRef<Path> + Send + Sync>,
) -> Result<()> {
    let filenames = collect_png_files(dirpath.as_ref())?;
    convert_parallel(filenames, output_dir)
}

/// Recursively collect all PNG files from a directory
fn collect_png_files(dir: &Path) -> Result<Vec<PathBuf>> {
    fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .try_fold(Vec::new(), |mut acc, entry| {
            let path = entry.path();
            if path.is_dir() {
                acc.extend(collect_png_files(&path)?);
            } else if path.extension().and_then(|s| s.to_str()) == Some("png") {
                acc.push(path);
            }
            Ok(acc)
        })
}

/// Convert multiple PNG files to SVG in parallel.
pub fn convert_parallel(
    filenames: Vec<impl AsRef<Path> + Send + 'static>,
    output_dir: Option<impl AsRef<Path> + Send + Sync>,
) -> Result<()> {
    filenames
        .into_par_iter()
        .try_for_each(|filename| convert(filename, output_dir.as_ref()))
}

/// Convert a single PNG file to SVG using VTracer algorithm.
pub fn convert(filename: impl AsRef<Path>, output_dir: Option<impl AsRef<Path>>) -> Result<()> {
    let input_path = filename.as_ref();

    // Read the image
    let img = image::open(input_path)?;

    // Convert to RGBA for VTracer
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    // Configure VTracer for high quality output
    let config = Config {
        color_mode: ColorMode::Color,
        hierarchical: Hierarchical::Stacked,
        filter_speckle: 4,
        color_precision: 6,
        layer_difference: 16,
        mode: PathSimplifyMode::Spline,
        corner_threshold: 60,
        length_threshold: 4.0,
        max_iterations: 10,
        splice_threshold: 45,
        path_precision: Some(2),
    };

    // Create ColorImage from raw data
    let img_data: Vec<u8> = rgba_img.into_raw();
    let color_image = ColorImage {
        pixels: img_data,
        width: width as usize,
        height: height as usize,
    };

    // Use VTracer to convert to SVG
    let svg_file = vtracer::convert(color_image, config).map_err(Error::RgbaConversionError)?;

    // Convert SvgFile to string using Display trait
    let svg = svg_file.to_string();

    // Determine output path
    let output_path = if let Some(dir) = output_dir {
        dir.as_ref()
            .join(input_path.file_stem().unwrap())
            .with_extension("svg")
    } else {
        input_path.with_extension("svg")
    };

    // Write SVG to file
    let mut file = File::create(&output_path)?;
    file.write_all(svg.as_bytes())?;

    Ok(())
}
