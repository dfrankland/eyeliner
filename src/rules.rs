use servo_css_parser::style::properties::declaration_block::PropertyDeclarationBlock;

#[derive(Clone)]
pub struct Rules {
    pub style: Vec<(String, PropertyDeclarationBlock)>,
    pub media: Vec<String>,
    pub font_face: Vec<String>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            style: vec![],
            media: vec![],
            font_face: vec![],
        }
    }
}
