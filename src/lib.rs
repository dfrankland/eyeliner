//! A CSS inliner for making emails.

mod rules;
mod hash;
mod options;
mod settings;
mod eyeliner;
pub mod traits;

pub use self::{
    rules::*,
    hash::*,
    options::*,
    settings::*,
    eyeliner::*,
};
use self::traits::*;

/// Returns a string of HTML with CSS inlined.
///
/// # Arguments
///
/// *   `html` - A string of HTML to have CSS inlined into. Any `<style />` tags will have their
///     styles parsed and processed.
///
/// *   `css` - An optional string of additional CSS to be inlined that is added _before_ the
///     `<style />` tags in the `html` are parsed.
///
/// *   `options` - An optional instance of `Options`.
///
/// *   `settings` - An optional instance of `Settings`.
///
/// # Remarks
///
/// Convenient function to inline HTML and CSS the same way as Juice.
///
/// # Examples
///
/// ```
///   use eyeliner::inline;
///
///   let html = r#"
///     <!DOCTYPE html>
///     <html>
///       <head>
///         <title>Test</title>
///       </head>
///       <body>
///         <h1>Hello, world!</h1>
///         <p>I <span class="red">love</span> Rust!</p>
///       </body>
///     </html>
///   "#;
///
///   let css = r#"
///     .red {
///       color: red;
///     }
///   "#;
///
///   let fixture = r#"
///     <!DOCTYPE html>
///     <html>
///       <head>
///         <title>Test</title>
///       </head>
///       <body>
///         <h1>Hello, world!</h1>
///         <p>I <span class="red" style="color: red;">love</span> Rust!</p>
///       </body>
///     </html>
///   "#;
///
///   assert_eq!(
///     inline(fixture, None, None, None), // Just used to format the HTML the same way
///     inline(html, Some(css.to_owned()), None, None),
///   );
/// ```
pub fn inline(
    html: &str,
    css: Option<String>,
    options: Option<AbstractOptions>,
    settings: Option<AbstractSettings>
) -> String {
    Eyeliner::new(html, css, options, settings)
        .collect_rules()
        .apply_rules()
        .apply_width_attributes()
        .apply_height_attributes()
        .apply_table_element_attributes()
        .insert_preserved_css()
        .to_string()
}
