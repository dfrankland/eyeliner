pub trait InlineStylesheetAndDocument {
    fn inline_stylesheet_and_document(self: &Self) -> &Self;
}

pub trait SerializeDocument {
    fn serialize_document(self: &Self) -> String;
}
