pub trait CollectRules {
    fn collect_rules(self: &mut Self) -> &mut Self;
}

pub trait ApplyRules {
    fn apply_rules(self: &mut Self) -> &mut Self;
}

pub trait ApplyAttributes {
    fn apply_attributes(self: &Self, property: &str) -> &Self;
}

pub trait ApplyWidthAttributes {
    fn apply_width_attributes(self: &Self) -> &Self;
}

pub trait ApplyHeightAttributes {
    fn apply_height_attributes(self: &Self) -> &Self;
}

pub trait ApplyTableElementAttributes {
    fn apply_table_element_attributes(self: &Self) -> &Self;
}

pub trait InsertPreservedCss {
    fn insert_preserved_css(self: &Self) -> &Self;
}

pub trait ToString {
    fn to_string(self: &Self) -> String;
}
