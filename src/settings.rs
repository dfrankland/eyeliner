use std::collections::HashMap;

pub mod default {
    use std::collections::HashMap;

    /// Settings referenced by features enabled through `Options`.
    #[derive(Clone, Debug)]
    pub struct Settings {
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
        pub width_elements: Option<Vec<String>>,

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
        pub height_elements: Option<Vec<String>>,

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
        pub style_to_attribute: Option<HashMap<String, String>>,

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
        pub table_elements: Option<Vec<String>>,

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
        pub non_visual_elements: Option<Vec<String>>,

        /// List of CSS style properties that will not be inlined.
        ///
        /// Defaults to:
        ///
        /// ```
        /// Vec::<&str>::new();
        /// ```
        ///
        pub excluded_properties: Option<Vec<String>>,
    }

    impl<'a> Default for Settings {
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
pub struct Settings {
    pub width_elements: Vec<String>,
    pub height_elements: Vec<String>,
    pub style_to_attribute: HashMap<String, String>,
    pub table_elements: Vec<String>,
    pub non_visual_elements: Vec<String>,
    pub excluded_properties: Vec<String>,
}

impl Settings {
    /// Takes the other `Settings` and uses any fields set on it or defaults to another value.
    pub fn new(opt: default::Settings) -> Self {
        Self {
            width_elements: opt.width_elements.unwrap_or_else(||
                vec![
                    "table".to_owned(),
                    "td".to_owned(),
                    "img".to_owned(),
                ]
            ),
            height_elements: opt.height_elements.unwrap_or_else(||
                vec![
                    "table".to_owned(),
                    "td".to_owned(),
                    "img".to_owned(),
                ]
            ),
            table_elements: opt.table_elements.unwrap_or_else(||
                vec![
                    "table".to_owned(),
                    "td".to_owned(),
                    "th".to_owned(),
                    "tr".to_owned(),
                    "td".to_owned(),
                    "caption".to_owned(),
                    "colgroup".to_owned(),
                    "col".to_owned(),
                    "thead".to_owned(),
                    "tbody".to_owned(),
                    "tfoot".to_owned(),
                ]
            ),
            style_to_attribute: opt.style_to_attribute.unwrap_or(
                hashmap!{
                    "background-color".to_owned() => "bgcolor".to_owned(),
                    "background-image".to_owned() => "background".to_owned(),
                    "text-align".to_owned() => "align".to_owned(),
                    "vertical-align".to_owned() => "valign".to_owned(),
                }
            ),
            non_visual_elements: opt.non_visual_elements.unwrap_or_else(||
                vec![
                    "head".to_owned(),
                    "title".to_owned(),
                    "base".to_owned(),
                    "link".to_owned(),
                    "style".to_owned(),
                    "meta".to_owned(),
                    "script".to_owned(),
                    "noscript".to_owned(),
                ]
            ),
            excluded_properties: opt.excluded_properties.unwrap_or_else(|| vec![]),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(default::Settings::default())
    }
}
