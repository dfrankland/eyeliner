extern crate kuchiki;
extern crate servo_css_parser;

mod rules;
mod hash;

mod eyeliner;
pub use eyeliner::*;

pub mod traits;
pub use eyeliner::*;

use traits::*;

pub fn inline<'a>(html: &'a str, css: &'a str) -> String {
    Eyeliner::new(html, css).inline_stylesheet_and_document().serialize_document()
}
