pub trait CollectRules {
    fn collect_rules(self: &mut Self) -> &mut Self;
}

pub trait ApplyRules {
    fn apply_rules(self: &mut Self) -> &mut Self;
}

pub trait ApplyWidthAttributes {
    fn apply_width_attributes(self: &Self) -> &Self;
}

pub trait ApplyHeightAttributes {
    fn apply_height_attributes(self: &Self) -> &Self;
}

pub trait ApplyAttributesTableElements {
    fn apply_attributes_table_elements(self: &Self) -> &Self;
}

pub trait InsertPreservedCss {
    fn insert_preserved_css(self: &Self) -> &Self;
}

pub trait SerializeDocument {
    fn serialize_document(self: &Self) -> String;
}
