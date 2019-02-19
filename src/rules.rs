use servo_css_parser::style::properties::declaration_block::PropertyDeclarationBlock;

/// Data collected from the CSS stylesheet.
#[derive(Clone, Debug)]
pub struct Rules {
    /// Style rules.
    pub style: Vec<(String, PropertyDeclarationBlock)>,

    /// `@media` rules.
    pub media: Vec<String>,

    /// `@font-face` rules.
    pub font_face: Vec<String>,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            style: vec![],
            media: vec![],
            font_face: vec![],
        }
    }
}
