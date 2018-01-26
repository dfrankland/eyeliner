use std::collections::HashMap;

pub mod default {
    use std::collections::HashMap;

    /// Settings referenced by features enabled through `Options`.
    #[derive(Clone, Debug)]
    pub struct Settings<'a> {
        /// List of HTML elements that can receive `width` attributes.
        ///
        /// Defaults to:
        ///
        /// ```
        /// vec![
        ///     "table",
        ///     "td",
        ///     "img",
        /// ];
        /// ```
        ///
        pub width_elements: Option<Vec<&'a str>>,

        /// List of HTML elements that can receive `height` attributes.
        ///
        /// Defaults to:
        ///
        /// ```
        /// vec![
        ///     "table",
        ///     "td",
        ///     "img",
        /// ];
        /// ```
        ///
        pub height_elements: Option<Vec<&'a str>>,

        /// Map of style property names to their respective attribute names.
        ///
        /// Defaults to:
        ///
        /// ```
        /// # #[macro_use] extern crate maplit;
        /// # fn main() {
        /// hashmap!{
        ///     "background-color" => "bgcolor",
        ///     "background-image" => "background",
        ///     "text-align" => "align",
        ///     "vertical-align" => "valign",
        /// };
        /// # }
        /// ```
        ///
        pub style_to_attribute: Option<HashMap<&'a str, &'a str>>,

        /// List of table HTML elements that can receive attributes defined in
        /// `Settings.style_to_attribute`.
        ///
        /// Defaults to:
        ///
        /// ```
        /// vec![
        ///     "table",
        ///     "td",
        ///     "th",
        ///     "tr",
        ///     "td",
        ///     "caption",
        ///     "colgroup",
        ///     "col",
        ///     "thead",
        ///     "tbody",
        ///     "tfoot",
        /// ];
        /// ```
        ///
        pub table_elements: Option<Vec<&'a str>>,

        /// List of elements that will not have styles inlined because they are not intended to
        /// render.
        ///
        /// Defaults to:
        ///
        /// ```
        /// vec![
        ///     "head",
        ///     "title",
        ///     "base",
        ///     "link",
        ///     "style",
        ///     "meta",
        ///     "script",
        ///     "noscript",
        /// ];
        /// ```
        ///
        pub non_visual_elements: Option<Vec<&'a str>>,

        /// List of CSS style properties that will not be inlined.
        ///
        /// Defaults to:
        ///
        /// ```
        /// Vec::<&str>::new();
        /// ```
        ///
        pub excluded_properties: Option<Vec<&'a str>>,
    }

    impl<'a> Default for Settings<'a> {
        fn default() -> Self {
            Self {
                width_elements: None,
                height_elements: None,
                style_to_attribute: None,
                table_elements: None,
                non_visual_elements: None,
                excluded_properties: None,
            }
        }
    }
}

/// The required settings to inline HTML and CSS. Use the other `Settings`, it is has nice
/// defaults.
#[derive(Clone, Debug)]
pub struct Settings<'a> {
    pub width_elements: Vec<&'a str>,
    pub height_elements: Vec<&'a str>,
    pub style_to_attribute: HashMap<&'a str, &'a str>,
    pub table_elements: Vec<&'a str>,
    pub non_visual_elements: Vec<&'a str>,
    pub excluded_properties: Vec<&'a str>,
}

impl<'a> Settings<'a> {
    /// Takes the other `Settings` and uses any fields set on it or defaults to another value.
    pub fn new(opt: default::Settings<'a>) -> Self {
        Self {
            width_elements: opt.width_elements.unwrap_or_else(||
                vec![
                    "table",
                    "td",
                    "img",
                ]
            ),
            height_elements: opt.height_elements.unwrap_or_else(||
                vec![
                    "table",
                    "td",
                    "img",
                ]
            ),
            table_elements: opt.table_elements.unwrap_or_else(||
                vec![
                    "table",
                    "td",
                    "th",
                    "tr",
                    "td",
                    "caption",
                    "colgroup",
                    "col",
                    "thead",
                    "tbody",
                    "tfoot",
                ]
            ),
            style_to_attribute: opt.style_to_attribute.unwrap_or(
                hashmap!{
                    "background-color" => "bgcolor",
                    "background-image" => "background",
                    "text-align" => "align",
                    "vertical-align" => "valign",
                }
            ),
            non_visual_elements: opt.non_visual_elements.unwrap_or_else(||
                vec![
                    "head",
                    "title",
                    "base",
                    "link",
                    "style",
                    "meta",
                    "script",
                    "noscript",
                ]
            ),
            excluded_properties: opt.excluded_properties.unwrap_or_else(|| vec![]),
        }
    }
}

impl<'a> Default for Settings<'a> {
    fn default() -> Self {
        Self::new(default::Settings::default())
    }
}
