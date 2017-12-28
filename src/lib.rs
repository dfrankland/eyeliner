extern crate kuchiki;
extern crate servo_css_parser;

pub mod eyeliner;
pub mod rules;
pub mod traits;
pub mod hash;

use traits::*;
use eyeliner::Eyeliner;

pub fn inline<'a>(html: &'a str, css: &'a str) -> String {
    Eyeliner::new(html, css).inline_stylesheet_and_document().serialize_document()
}
