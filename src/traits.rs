pub trait InlineStylesheetAndDocument {
    fn inline_stylesheet_and_document(self: &mut Self) -> &Self;
}

pub trait ApplyWidthAttributes {
    fn apply_width_attributes(self: &Self) -> &Self;
}

pub trait ApplyHeightAttributes {
    fn apply_height_attributes(self: &Self) -> &Self;
}

pub trait SerializeDocument {
    fn serialize_document(self: &Self) -> String;
}
