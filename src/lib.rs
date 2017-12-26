extern crate kuchiki;
extern crate servo_css_parser;
#[macro_use] extern crate markup5ever;

pub mod eyeliner;

use eyeliner::{Eyeliner, InlineStylesheetAndDocument};

pub fn inline<'a>(html: &'a str, css: &'a str) -> String {
    let eyeliner = Eyeliner::new(html, css);
    eyeliner.inline_stylesheet_and_document()
}
