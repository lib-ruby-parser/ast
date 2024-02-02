use lib_ruby_parser_nodes::{reexports::liquid::value, LiquidTemplate};

pub(crate) fn codegen() {
    /* Node test helper */
    let template = LiquidTemplate::new("codegen/loc_name.liquid").with_global(
        "loc_names",
        value!(vec![
            "begin",
            "end",
            "expression",
            "keyword",
            "name",
            "assignment",
            "colon",
            "double_colon",
            "else",
            "heredoc_body",
            "operator",
            "selector",
            "assoc",
            "question",
            "heredoc_end",
        ]),
    );
    let rendered = template.render();
    std::fs::write("src/loc_name/gen.rs", rendered).unwrap();
}
