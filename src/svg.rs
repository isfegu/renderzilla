use usvg::TreeParsing;

pub fn serialize(svg_string: &str, convert_text: bool) -> Result<usvg::Tree, usvg::Error> {
    let mut usvg_tree = usvg::Tree::from_str(svg_string, &usvg::Options::default())?;

    if usvg_tree.has_text_nodes() && convert_text {
        text_to_path(&mut usvg_tree)?;
    }

    Ok(usvg_tree)
}

fn text_to_path(usvg_tree: &mut usvg::Tree) -> Result<(), usvg::Error> {
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();
    usvg::TreeTextToPath::convert_text(usvg_tree, &fontdb);

    Ok(())
}
