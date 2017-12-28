use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use kuchiki::traits::*;
use kuchiki::{NodeRef, parse_html};

use servo_css_parser::parse;
use servo_css_parser::types::{Url, QuirksMode, MediaList, Origin, ServoStylesheet};
use servo_css_parser::style::stylesheets::{CssRules, CssRule, StyleRule};
use servo_css_parser::style::properties::declaration_block::{PropertyDeclarationBlock, DeclarationSource, Importance};
use servo_css_parser::style::properties::BuilderArc as Arc;
use servo_css_parser::style::shared_lock::Locked;

use traits::*;
use hash::HashableNodeRef;
use rules::EyelinerRules;

fn parse_css(css: &str) -> ServoStylesheet {
    let url = Url::parse("about::test").unwrap();
    let origin = Origin::UserAgent;
    let quirks_mode = QuirksMode::NoQuirks;
    let media = MediaList::empty();

    parse(css, url, origin, quirks_mode, media)
}

pub struct Eyeliner {
    pub document: NodeRef,
    pub stylesheet: ServoStylesheet,
}

impl Eyeliner {
    pub fn new(html: &str, css: &str) -> Self {
        Self {
            document: parse_html().one(html),
            stylesheet: parse_css(css),
        }
    }
}

impl StylesheetAsEyelinerRules for Eyeliner {
    fn stylesheet_as_eyeliner_rules(self: &Self, rules: &Arc<Locked<CssRules>>) -> EyelinerRules {
        let mut eyeliner_rules = EyelinerRules::new();

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

impl InlineStylesheetAndDocument for Eyeliner {
    fn inline_stylesheet_and_document(self: &Self) -> &Self {
        let eyeliner_rules = self.stylesheet_as_eyeliner_rules(&self.stylesheet.contents.rules);

        let mut node_style_map: HashMap<HashableNodeRef, (PropertyDeclarationBlock, Option<String>)> = HashMap::new();

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
                        entry.get_mut().0.extend_from_block(&block);
                        entry.get().clone()
                    },

                    Vacant(entry) => {
                        let exisiting_style = match attributes.get("style") {
                            Some(style) => Some(style.to_string()),
                            None => None,
                        };
                        let new_style = (block.clone(), exisiting_style);
                        entry.insert(new_style.clone());
                        new_style
                    },
                };

                use servo_css_parser::style_traits::values::ToCss;
                let new_inlined_css = css.0.to_css_string();

                attributes.insert("style", match &css.1 {
                    &Some(ref existing_inlined_css) => {
                        let delimeter = match true {
                            _ if existing_inlined_css.ends_with("; ") => "",
                            _ if existing_inlined_css.ends_with(";") => " ",
                            _ => "; ",
                        };
                        format!("{}{}{}", existing_inlined_css, delimeter, new_inlined_css)
                    },
                    &None => new_inlined_css,
                });
            }
        }

        self
    }
}

impl SerializeDocument for Eyeliner {
    fn serialize_document(self: &Self) -> String {
        let mut writer = vec![];
        self.document.serialize(&mut writer).unwrap();
        String::from_utf8_lossy(&writer).into_owned()
    }
}
