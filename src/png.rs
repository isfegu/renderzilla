use std::error::Error;

pub fn encode(usvg_tree: &usvg::Tree) -> Result<Vec<u8>, Box<dyn Error>> {
    let resvg_tree = resvg::Tree::from_usvg(usvg_tree);

    let pixmap_size = resvg_tree.size;
    let mut pixmap =
        resvg::tiny_skia::Pixmap::new(pixmap_size.width() as u32, pixmap_size.height() as u32)
            .ok_or("failed to create new pixmap")?;
    resvg_tree.render(usvg::Transform::default(), &mut pixmap.as_mut());

    let png_data = pixmap.encode_png()?;

    Ok(png_data)
}
