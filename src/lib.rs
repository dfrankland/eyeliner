extern crate kuchiki;
extern crate servo_css_parser;
#[macro_use] extern crate html5ever;
#[macro_use] extern crate maplit;

mod rules;
mod hash;
mod property_declaration_value;

mod options;
pub use options::default::*;

mod settings;
pub use settings::default::*;

mod eyeliner;
pub use eyeliner::*;

pub mod traits;
pub use eyeliner::*;

use traits::*;

pub fn inline(html: &str, css: Option<&str>, options: Option<Options>, settings: Option<Settings>) -> String {
    Eyeliner::new(html, css, options, settings)
        .get_stylesheet_as_eyeliner_rules()
        .inline_stylesheet_and_document()
        .apply_width_attributes()
        .apply_height_attributes()
        .apply_attributes_table_elements()
        .insert_preserved_css()
        .serialize_document()
}
