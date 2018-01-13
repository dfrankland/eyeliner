pub trait InlineStylesheetAndDocument {
    fn inline_stylesheet_and_document(self: &mut Self) -> &Self;
}

pub trait SerializeDocument {
    fn serialize_document(self: &Self) -> String;
}
