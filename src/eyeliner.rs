use kuchiki::traits::*;
use kuchiki::{NodeRef, parse_html};

use servo_css_parser::parse;
use servo_css_parser::types::{Url, QuirksMode, MediaList, Origin, ServoStylesheet};
use servo_css_parser::style::stylesheets::{CssRules, CssRule, StyleRule};
use servo_css_parser::style::properties::declaration_block::{PropertyDeclarationBlock, DeclarationSource, Importance};
use servo_css_parser::selectors::parser::{SelectorList, Selector, SelectorImpl};

use markup5ever::interface::QualName;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub fn parse_css(css: &str) -> ServoStylesheet {
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

// Extend `SelectorList` to expose inner `Selector`s
// TODO: try to upstream this; not sure why it wouldn't be included.

pub trait SelectorListExt<Impl: SelectorImpl> {
    fn to_vec(&self) -> Vec<Selector<Impl>>;
}

impl<Impl: SelectorImpl> SelectorListExt<Impl> for SelectorList<Impl> {
    fn to_vec(&self) -> Vec<Selector<Impl>> {
        let &SelectorList (ref smallvec) = self;
        smallvec.to_vec()
    }
}

pub trait CssRulesExt {
    fn to_vec(&self) -> Vec<CssRule>;
}

impl CssRulesExt for CssRules {
    fn to_vec(&self) -> Vec<CssRule> {
        let &CssRules (ref rules) = self;
        rules.clone()
    }
}

// Create a hashmap from css rules

pub trait StylesheetToHashMap {
    fn stylesheet_to_hashmap(self: &Self) -> HashMap<String, PropertyDeclarationBlock>;
}

impl StylesheetToHashMap for Eyeliner {
    fn stylesheet_to_hashmap(self: &Self) -> HashMap<String, PropertyDeclarationBlock> {
        let mut css_map = HashMap::<String, PropertyDeclarationBlock>::new();

        let read_guard = &self.stylesheet.shared_lock.read();

        let css_rules = &self.stylesheet.contents.rules.as_ref().read_with(read_guard).to_vec();

        for rule in css_rules {
            #[allow(unknown_lints)]
            #[allow(single_match)]
            match *rule {
                CssRule::Style (ref style_rule_locked) => {
                    let style_rule = style_rule_locked.as_ref().read_with(read_guard);

                    #[allow(unknown_lints)]
                    #[allow(single_match)]
                    match *style_rule {
                        StyleRule { ref selectors, block: ref block_locked, .. } => {
                            let block = block_locked.as_ref().read_with(read_guard);

                            for selector in selectors.to_vec() {
                                use servo_css_parser::cssparser::ToCss;

                                match css_map.entry(selector.to_css_string()) {
                                    Occupied(mut entry) => {
                                        for (i, declartion) in block.declarations().into_iter().enumerate() {
                                            #[allow(unknown_lints)]
                                            #[allow(match_bool)]
                                            let importance = if block.declarations_importance().get(i as u32) {
                                                Importance::Important
                                            } else {
                                                Importance::Normal
                                            };
                                            entry.get_mut().push(declartion.clone(), importance, DeclarationSource::Parsing);
                                        }
                                    },
                                    Vacant(entry) => {
                                        entry.insert(block.clone());
                                    }
                                }
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }

        css_map
    }
}

pub trait InlineStylesheetAndDocument {
    fn inline_stylesheet_and_document(self: &Self) -> String;
}

impl InlineStylesheetAndDocument for Eyeliner {
    fn inline_stylesheet_and_document(self: &Self) -> String {
        let css = self.stylesheet_to_hashmap();

        for (selector, block) in css {
            for css_match in self.document.select(&selector).unwrap() {
                let mut attributes = css_match.attributes.borrow_mut();
                let style_attr = QualName::new(
                    None,
                    ns!(),
                    local_name!("style"),
                );

                use servo_css_parser::style_traits::values::ToCss;

                match attributes.map.entry(style_attr) {
                    Occupied(mut entry) => {
                        let exising_style = entry.get().clone();
                        let delimeter = match true {
                            _ if exising_style.ends_with("; ") => "",
                            _ if exising_style.ends_with(";") => " ",
                            _ => "; ",
                        };
                        entry.insert(format!("{}{}{}", exising_style, delimeter, block.to_css_string()));
                    },
                    Vacant(entry) => {
                        entry.insert(block.to_css_string());
                    },
                }

                let class_attr = QualName::new(
                    None,
                    ns!(),
                    local_name!("class"),
                );

                match attributes.map.entry(class_attr) {
                    Occupied(mut entry) => {
                        entry.remove_entry();
                    },
                    _ => {},
                }
            }
        }

        let mut writer = vec![];

        self.document.serialize(&mut writer).unwrap();

        String::from_utf8_lossy(&writer).into_owned()
    }
}
