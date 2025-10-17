use png2svg_core::convert;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_name = "../9999.png";
    println!("Processing PNG file: {}", file_name);

    convert(file_name, None::<String>)?;
    Ok(())
}
