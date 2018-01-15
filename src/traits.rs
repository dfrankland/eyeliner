//! All of the traits implemented by `Eyeliner`.

/// Collect CSS rules from the CSS stylesheet for other methods to use.
pub trait CollectRules {
    fn collect_rules(self: &mut Self) -> &mut Self;
}

/// Inline CSS rules extracted from the CSS stylesheet into the HTML document.
pub trait ApplyRules {
    fn apply_rules(self: &mut Self) -> &mut Self;
}

/// Apply a specified CSS property as an attribute to elements.
pub trait ApplyAttributes {
    fn apply_attributes(self: &Self, property: &str) -> &Self;
}

/// Apply a width attribute to elements.
pub trait ApplyWidthAttributes {
    fn apply_width_attributes(self: &Self) -> &Self;
}

/// Apply a height attribute to elements.
pub trait ApplyHeightAttributes {
    fn apply_height_attributes(self: &Self) -> &Self;
}

/// Apply a table attributes to table elements.
pub trait ApplyTableElementAttributes {
    fn apply_table_element_attributes(self: &Self) -> &Self;
}

/// Insert preserved CSS rules as a `<style />` tag to a specified node.
pub trait InsertPreservedCss {
    fn insert_preserved_css(self: &Self) -> &Self;
}
