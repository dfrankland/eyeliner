use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use kuchiki::traits::*;
use kuchiki::{NodeRef, parse_html};

use servo_css_parser::parse;
use servo_css_parser::types::{Url, QuirksMode, MediaList, Origin, ServoStylesheet as Stylesheet};
use servo_css_parser::style::stylesheets::{CssRules, CssRule, StyleRule};
use servo_css_parser::style::properties::declaration_block::{parse_style_attribute, PropertyDeclarationBlock, DeclarationSource, Importance};
use servo_css_parser::style::properties::BuilderArc as Arc;
use servo_css_parser::style::shared_lock::Locked;
use servo_css_parser::style::error_reporting::RustLogReporter;

use traits::*;
use hash::HashableNodeRef;
use rules::Rules;
use options::{Options, default as default_options};
use settings::{Settings, default as default_settings};

pub struct Eyeliner<'a> {
    pub document: NodeRef,
    pub stylesheet: Stylesheet,
    pub options: Options<'a>,
    pub settings: Settings<'a>,
}

impl<'a> Eyeliner<'a> {
    pub fn new(
        html: &str,
        css: Option<&str>,
        options: Option<default_options::Options<'a>>,
        settings: Option<default_settings::Settings<'a>>,
    ) -> Self {
        let options = Options::new(options.unwrap_or(default_options::Options::default()));
        let settings = Settings::new(settings.unwrap_or(default_settings::Settings::default()));

        let mut css = css.unwrap_or("").to_owned();
        let document = parse_html().one(html);

        if options.apply_style_tags {
            for node in document.select("style").unwrap() {
                css += &node.text_contents();
                if options.remove_style_tags {
                    node.as_node().detach();
                }
            }
        }

        let url = Url::parse("about::test").unwrap();
        let origin = Origin::UserAgent;
        let quirks_mode = QuirksMode::NoQuirks;
        let media = MediaList::empty();
        let stylesheet = parse(&css, url, origin, quirks_mode, media);

        Self {
            document: document,
            stylesheet: stylesheet,
            options: options,
            settings: settings,
        }
    }

    fn stylesheet_as_eyeliner_rules(self: &Self, rules: &Arc<Locked<CssRules>>) -> Rules {
        let mut eyeliner_rules = Rules::new();

        let read_guard = &self.stylesheet.shared_lock.read();

        let css_rules = &rules.as_ref().read_with(read_guard).0;
        for css_rule in css_rules {
            match *css_rule {
                CssRule::Style (ref style_rule_locked) => {
                    let style_rule = style_rule_locked.as_ref().read_with(read_guard);
                    let StyleRule { ref selectors, block: ref block_locked, .. } = *style_rule;

                    use servo_css_parser::cssparser::ToCss;
                    eyeliner_rules.style.push((
                        selectors.to_css_string(),
                        block_locked.as_ref().read_with(read_guard).clone(),
                    ));
                },

                CssRule::Media (ref media_rule_locked) => {
                    let media_rule = media_rule_locked.as_ref().read_with(read_guard);

                    use servo_css_parser::style::shared_lock::ToCssWithGuard;
                    eyeliner_rules.media.push(media_rule.to_css_string(read_guard));
                },

                CssRule::FontFace (ref font_face_rule_data_locked) => {
                    let font_face_rule_data = font_face_rule_data_locked.as_ref().read_with(read_guard);

                    use servo_css_parser::style::shared_lock::ToCssWithGuard;
                    eyeliner_rules.font_face.push(font_face_rule_data.to_css_string(read_guard));
                },

                _ => {},
            }
        }

        eyeliner_rules
    }
}

trait ExtendFromPropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &Self;
}
impl ExtendFromPropertyDeclarationBlock for PropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &Self {
        for (i, declartion) in block.declarations().into_iter().enumerate() {
            let importance = if block.declarations_importance().get(i as u32) {
                Importance::Important
            } else {
                Importance::Normal
            };
            self.push(declartion.clone(), importance, DeclarationSource::Parsing);
        }

        self
    }
}

impl<'a> InlineStylesheetAndDocument for Eyeliner<'a> {
    fn inline_stylesheet_and_document(self: &Self) -> &Self {
        let eyeliner_rules = self.stylesheet_as_eyeliner_rules(&self.stylesheet.contents.rules);

        let mut node_style_map: HashMap<HashableNodeRef, PropertyDeclarationBlock> = HashMap::new();

        for (selector, block) in eyeliner_rules.style {

            // TODO: using `::` seems to break things.
            // While testing using Bootstrap CSS, `::after` and `::before` give stack overflows.
            if selector.contains("::after") || selector.contains("::before") {
                continue;
            }

            for node in self.document.select(&selector).unwrap() {
                let mut attributes = node.attributes.borrow_mut();

                let css = match node_style_map.entry(HashableNodeRef::new(&node)) {
                    Occupied(mut entry) => {
                        entry.get_mut().extend_from_block(&block);
                        entry.get().clone()
                    },

                    Vacant(entry) => {
                        let mut exisiting_style = parse_style_attribute(
                            &attributes.get("style").unwrap_or(""),
                            &self.stylesheet.contents.url_data.read(),
                            &RustLogReporter {},
                            QuirksMode::NoQuirks,
                        );
                        exisiting_style.extend_from_block(&block);
                        entry.insert(exisiting_style.clone());
                        exisiting_style
                    },
                };

                use servo_css_parser::style_traits::values::ToCss;
                attributes.insert("style", css.to_css_string());
            }
        }

        self
    }
}

impl<'a> SerializeDocument for Eyeliner<'a> {
    fn serialize_document(self: &Self) -> String {
        let mut writer = vec![];
        self.document.serialize(&mut writer).unwrap();
        String::from_utf8_lossy(&writer).into_owned()
    }
}
