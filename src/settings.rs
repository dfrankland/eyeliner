use std::collections::HashMap;

use servo_css_parser::style::properties::PropertyDeclaration;
use servo_css_parser::style::values::specified::color::Color;
use servo_css_parser::style::properties::longhands::background_image::SpecifiedValue as Image;
use servo_css_parser::style::values::{Either, None_};
use servo_css_parser::style::values::specified::text::{TextAlign, TextAlignKeyword};
use servo_css_parser::style::values::generics::box_::VerticalAlign as GenericVerticalAlign;

use hash::HashablePropertyDeclaration;

pub mod default {
    use std::collections::HashMap;

    use hash::HashablePropertyDeclaration;

    pub struct Settings<'a> {
        pub width_elements: Option<Vec<&'a str>>,
        pub height_elements: Option<Vec<&'a str>>,
        pub style_to_attribute: Option<HashMap<HashablePropertyDeclaration, &'a str>>,
        pub table_elements: Option<Vec<&'a str>>,
        pub non_visual_elements: Option<Vec<&'a str>>,
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


pub struct Settings<'a> {
    pub width_elements: Vec<&'a str>,
    pub height_elements: Vec<&'a str>,
    pub style_to_attribute: HashMap<HashablePropertyDeclaration, &'a str>,
    pub table_elements: Vec<&'a str>,
    pub non_visual_elements: Vec<&'a str>,
    pub excluded_properties: Vec<&'a str>,
}

impl<'a> Settings<'a> {
    pub fn new(opt: default::Settings<'a>) -> Self {
        Self {
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
                    HashablePropertyDeclaration::new(
                        PropertyDeclaration::BackgroundColor(Color::transparent())
                    ) => "bgcolor",
                    HashablePropertyDeclaration::new(
                        PropertyDeclaration::BackgroundImage(Image(vec![Either::First(None_)]))
                    ) => "background",
                    HashablePropertyDeclaration::new(
                        PropertyDeclaration::TextAlign(TextAlign::Keyword(TextAlignKeyword::start()))
                    ) => "align",
                    HashablePropertyDeclaration::new(
                        PropertyDeclaration::VerticalAlign(GenericVerticalAlign::baseline())
                    ) => "valign",
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
