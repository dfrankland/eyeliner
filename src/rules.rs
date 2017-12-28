use servo_css_parser::style::properties::declaration_block::PropertyDeclarationBlock;

#[derive(Clone)]
pub struct EyelinerRules {
    pub style: Vec<(String, PropertyDeclarationBlock)>,
    pub media: Vec<String>,
    pub font_face: Vec<String>,
}

impl EyelinerRules {
    pub fn new() -> Self {
        EyelinerRules {
            style: vec![],
            media: vec![],
            font_face: vec![],
        }
    }
}
