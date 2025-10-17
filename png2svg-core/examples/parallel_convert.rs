use png2svg_core::convert;
use rayon::prelude::*;
use std::{env, fs, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let dir_path = if args.len() > 1 {
        Path::new(&args[1])
    } else {
        Path::new(".")
    };

    if !dir_path.is_dir() {
        eprintln!("Error: The provided path is not a directory.");
        std::process::exit(1);
    }

    println!("Processing PNG files in: {}", dir_path.display());

    fs::read_dir(dir_path)?
        .filter_map(Result::ok)
        .collect::<Vec<_>>()
        .par_iter()
        .for_each(|entry| {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("png") {
                let input_path = path.to_str().unwrap();
                let output_path = path.with_extension("svg").to_str().unwrap().to_string();

                println!("Converting {} to {}", input_path, output_path);

                convert(input_path).unwrap();
            }
        });
    Ok(())
}
