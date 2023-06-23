use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let file_contents = std::fs::read_to_string("equation.svg")?;

    let svg_tree = render::svg::serialize(&file_contents, true)?;

    let encoded_png = render::png::encode(&svg_tree)?;

    std::fs::write("equation.png", &encoded_png)?;

    Ok(())
}
