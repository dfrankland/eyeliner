use servo_css_parser::style::properties::declaration_block::PropertyDeclarationBlock;

pub trait ExtendFromPropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &Self;
}

pub trait InlineStylesheetAndDocument {
    fn inline_stylesheet_and_document(self: &Self) -> &Self;
}

pub trait SerializeDocument {
    fn serialize_document(self: &Self) -> String;
}
