use crate::{Error, Result};
use image::{Rgba, RgbaImage};
use rayon::prelude::*;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

type Point = (i32, i32);
type Edge = (Point, Point);

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

/// Convert a single PNG file to SVG.
pub fn convert(filename: impl AsRef<Path>, output_dir: Option<impl AsRef<Path>>) -> Result<()> {
    let img = image::open(filename.as_ref())?;

    // Check if image has alpha channel
    let svg = match img.color() {
        image::ColorType::Rgba8
        | image::ColorType::Rgba16
        | image::ColorType::Rgba32F
        | image::ColorType::La8
        | image::ColorType::La16 => {
            // Has alpha channel, convert to RGBA
            let rgba_img = img.to_rgba8();
            rgba_image_to_svg_contiguous(&rgba_img)
        }
        image::ColorType::Rgb8
        | image::ColorType::Rgb16
        | image::ColorType::Rgb32F
        | image::ColorType::L8
        | image::ColorType::L16 => {
            // No alpha channel, convert to RGB
            let rgb_img = img.to_rgb8();
            rgb_image_to_svg_contiguous(&rgb_img)
        }
        _ => {
            return Err(Error::RgbaConversionError(format!(
                "Unsupported color type: {:?} for file: {}",
                img.color(),
                filename.as_ref().display()
            )));
        }
    };

    let input_path = filename.as_ref().to_path_buf();
    let output_path = if let Some(dir) = output_dir {
        dir.as_ref()
            .join(input_path.file_stem().unwrap())
            .with_extension("svg")
            .to_str()
            .unwrap()
            .to_string()
    } else {
        input_path
            .with_extension("svg")
            .to_str()
            .unwrap()
            .to_string()
    };

    let mut file = File::create(&output_path)?;
    file.write_all(svg.as_bytes())?;

    Ok(())
}

#[inline]
fn svg_header(width: u32, height: u32) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN"
  "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
<svg width="{}" height="{}"
     xmlns="http://www.w3.org/2000/svg" version="1.1">
"#,
        width, height
    )
}

fn rgba_image_to_svg_contiguous(img: &RgbaImage) -> String {
    let (width, height) = (img.width() as i32, img.height() as i32);
    let adjacent = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visited = vec![vec![false; height as usize]; width as usize];
    let mut color_pixel_lists: HashMap<Rgba<u8>, Vec<HashSet<Point>>> = HashMap::new();

    for x in 0..width as u32 {
        for y in 0..height as u32 {
            if visited[x as usize][y as usize] {
                continue;
            }
            let rgba = img.get_pixel(x, y);
            if rgba[3] == 0 {
                continue;
            }
            let mut piece = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((x as i32, y as i32));
            visited[x as usize][y as usize] = true;

            while let Some(here) = queue.pop_front() {
                for offset in &adjacent {
                    let neighbour = (here.0 + offset.0, here.1 + offset.1);
                    if neighbour.0 < 0
                        || neighbour.0 >= width
                        || neighbour.1 < 0
                        || neighbour.1 >= height
                    {
                        continue;
                    }
                    if visited[neighbour.0 as usize][neighbour.1 as usize] {
                        continue;
                    }
                    let neighbour_rgba = img.get_pixel(neighbour.0 as u32, neighbour.1 as u32);
                    if neighbour_rgba != rgba {
                        continue;
                    }
                    queue.push_back(neighbour);
                    visited[neighbour.0 as usize][neighbour.1 as usize] = true;
                }
                piece.insert(here);
            }

            color_pixel_lists.entry(*rgba).or_default().push(piece);
        }
    }

    let edges = [
        ((-1, 0), ((0, 0), (0, 1))),
        ((0, 1), ((0, 1), (1, 1))),
        ((1, 0), ((1, 1), (1, 0))),
        ((0, -1), ((1, 0), (0, 0))),
    ];

    let mut color_edge_lists: HashMap<Rgba<u8>, Vec<HashSet<Edge>>> = HashMap::new();

    for (rgba, pieces) in &color_pixel_lists {
        for piece_pixel_list in pieces {
            let mut edge_set = HashSet::new();
            for &coord in piece_pixel_list {
                for &(offset, (start_offset, end_offset)) in &edges {
                    let neighbour = (coord.0 + offset.0, coord.1 + offset.1);
                    let start = (coord.0 + start_offset.0, coord.1 + start_offset.1);
                    let end = (coord.0 + end_offset.0, coord.1 + end_offset.1);
                    let edge = (start, end);
                    if !piece_pixel_list.contains(&neighbour) {
                        edge_set.insert(edge);
                    }
                }
            }
            color_edge_lists.entry(*rgba).or_default().push(edge_set);
        }
    }

    let mut svg = String::new();
    svg.push_str(&svg_header(img.width(), img.height()));

    for (color, pieces) in &color_edge_lists {
        for edge_set in pieces {
            let shape = joined_edges(edge_set, false);
            svg.push_str(r#" <path d=""#);
            for sub_shape in shape {
                if let Some(&start) = sub_shape.first() {
                    svg.push_str(&format!(" M {},{}", start.0, start.1));
                    for &point in &sub_shape[1..] {
                        svg.push_str(&format!(" L {},{}", point.0, point.1));
                    }
                    svg.push_str(" Z");
                }
            }
            svg.push_str(&format!(
                r#"" style="fill:rgb({},{},{}); fill-opacity:{}; stroke:none;" />"#,
                color[0],
                color[1],
                color[2],
                color[3] as f32 / 255.0
            ));
        }
    }

    svg.push_str("</svg>\n");
    svg
}

fn rgb_image_to_svg_contiguous(img: &image::RgbImage) -> String {
    let (width, height) = (img.width() as i32, img.height() as i32);
    let adjacent = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut visited = vec![vec![false; height as usize]; width as usize];
    let mut color_pixel_lists: HashMap<image::Rgb<u8>, Vec<HashSet<Point>>> = HashMap::new();

    for x in 0..width as u32 {
        for y in 0..height as u32 {
            if visited[x as usize][y as usize] {
                continue;
            }
            let rgb = img.get_pixel(x, y);
            let mut piece = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((x as i32, y as i32));
            visited[x as usize][y as usize] = true;

            while let Some(here) = queue.pop_front() {
                for offset in &adjacent {
                    let neighbour = (here.0 + offset.0, here.1 + offset.1);
                    if neighbour.0 < 0
                        || neighbour.0 >= width
                        || neighbour.1 < 0
                        || neighbour.1 >= height
                    {
                        continue;
                    }
                    if visited[neighbour.0 as usize][neighbour.1 as usize] {
                        continue;
                    }
                    let neighbour_rgb = img.get_pixel(neighbour.0 as u32, neighbour.1 as u32);
                    if neighbour_rgb != rgb {
                        continue;
                    }
                    queue.push_back(neighbour);
                    visited[neighbour.0 as usize][neighbour.1 as usize] = true;
                }
                piece.insert(here);
            }

            color_pixel_lists.entry(*rgb).or_default().push(piece);
        }
    }

    let edges = [
        ((-1, 0), ((0, 0), (0, 1))),
        ((0, 1), ((0, 1), (1, 1))),
        ((1, 0), ((1, 1), (1, 0))),
        ((0, -1), ((1, 0), (0, 0))),
    ];

    let mut color_edge_lists: HashMap<image::Rgb<u8>, Vec<HashSet<Edge>>> = HashMap::new();

    for (rgb, pieces) in &color_pixel_lists {
        for piece_pixel_list in pieces {
            let mut edge_set = HashSet::new();
            for &coord in piece_pixel_list {
                for &(offset, (start_offset, end_offset)) in &edges {
                    let neighbour = (coord.0 + offset.0, coord.1 + offset.1);
                    let start = (coord.0 + start_offset.0, coord.1 + start_offset.1);
                    let end = (coord.0 + end_offset.0, coord.1 + end_offset.1);
                    let edge = (start, end);
                    if !piece_pixel_list.contains(&neighbour) {
                        edge_set.insert(edge);
                    }
                }
            }
            color_edge_lists.entry(*rgb).or_default().push(edge_set);
        }
    }

    let mut svg = String::new();
    svg.push_str(&svg_header(img.width(), img.height()));

    for (color, pieces) in &color_edge_lists {
        for edge_set in pieces {
            let shape = joined_edges(edge_set, false);
            svg.push_str(r#" <path d=""#);
            for sub_shape in shape {
                if let Some(&start) = sub_shape.first() {
                    svg.push_str(&format!(" M {},{}", start.0, start.1));
                    for &point in &sub_shape[1..] {
                        svg.push_str(&format!(" L {},{}", point.0, point.1));
                    }
                    svg.push_str(" Z");
                }
            }
            // RGB images are fully opaque (opacity = 1.0)
            svg.push_str(&format!(
                r#"" style="fill:rgb({},{},{}); fill-opacity:1.0; stroke:none;" />"#,
                color[0], color[1], color[2]
            ));
        }
    }

    svg.push_str("</svg>\n");
    svg
}

#[inline]
fn joined_edges(assorted_edges: &HashSet<Edge>, keep_every_point: bool) -> Vec<Vec<Point>> {
    let mut pieces = Vec::new();
    let mut assorted_edges = assorted_edges.clone();
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while !assorted_edges.is_empty() {
        let mut piece = Vec::new();
        let first_edge = *assorted_edges.iter().next().unwrap();
        assorted_edges.remove(&first_edge);
        piece.push(first_edge.0);
        piece.push(first_edge.1);

        loop {
            let last_point = *piece.last().unwrap();
            let mut found = false;

            for &direction in &directions {
                let next_point = (last_point.0 + direction.0, last_point.1 + direction.1);
                let next_edge = (last_point, next_point);

                if assorted_edges.contains(&next_edge) {
                    assorted_edges.remove(&next_edge);
                    if !keep_every_point && piece.len() >= 2 {
                        let prev_direction = (
                            piece[piece.len() - 1].0 - piece[piece.len() - 2].0,
                            piece[piece.len() - 1].1 - piece[piece.len() - 2].1,
                        );
                        if prev_direction == direction {
                            piece.pop();
                        }
                    }
                    piece.push(next_point);
                    found = true;
                    break;
                }
            }

            if !found || piece.first() == piece.last() {
                break;
            }
        }

        if piece.first() == piece.last() {
            piece.pop();
        }
        pieces.push(piece);
    }

    pieces
}
