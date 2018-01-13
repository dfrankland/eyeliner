use std::collections::HashMap;

pub mod default {
    use std::collections::HashMap;

    pub struct Settings<'a> {
        pub ignored_pseudos: Option<Vec<&'a str>>,
        pub width_elements: Option<Vec<&'a str>>,
        pub height_elements: Option<Vec<&'a str>>,
        pub style_to_attribute: Option<HashMap<&'a str, &'a str>>,
        pub table_elements: Option<Vec<&'a str>>,
        pub non_visual_elements: Option<Vec<&'a str>>,
        pub excluded_properties: Option<Vec<&'a str>>,
    }

    impl<'a> Default for Settings<'a> {
        fn default() -> Self {
            Self {
                ignored_pseudos: None,
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


pub struct Settings<'a> {
    pub ignored_pseudos: Vec<&'a str>,
    pub width_elements: Vec<&'a str>,
    pub height_elements: Vec<&'a str>,
    pub style_to_attribute: HashMap<&'a str, &'a str>,
    pub table_elements: Vec<&'a str>,
    pub non_visual_elements: Vec<&'a str>,
    pub excluded_properties: Vec<&'a str>,
}

impl<'a> Settings<'a> {
    pub fn new(opt: default::Settings<'a>) -> Self {
        Self {
            ignored_pseudos: opt.ignored_pseudos.unwrap_or(
                vec![
                    "hover",
                    "active",
                    "focus",
                    "visited",
                    "link",
                ]
            ),
            width_elements: opt.width_elements.unwrap_or(
                vec![
                    "table",
                    "td",
                    "img",
                ]
            ),
            height_elements: opt.height_elements.unwrap_or(
                vec![
                    "table",
                    "td",
                    "img",
                ]
            ),
            table_elements: opt.table_elements.unwrap_or(
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
                    "tfoot"
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
            non_visual_elements: opt.non_visual_elements.unwrap_or(
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
            excluded_properties: opt.excluded_properties.unwrap_or(vec![]),
        }
    }
}
