use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use kuchiki::traits::*;
use kuchiki::{parse_html, NodeRef};

use html5ever::QualName;

use servo_css_parser::parse;
use servo_css_parser::types::{Url, QuirksMode, MediaList, Origin, ServoStylesheet as Stylesheet};
use servo_css_parser::style::stylesheets::{CssRule, StyleRule};
use servo_css_parser::style::properties::declaration_block::{parse_style_attribute, PropertyDeclarationBlock, DeclarationSource, Importance};
use servo_css_parser::style::properties::{PropertyDeclarationId, PropertyId};
use servo_css_parser::style::error_reporting::RustLogReporter;

use traits::*;
use hash::HashableNodeRef;
use rules::Rules;
use options::{Options, default as default_options};
use settings::{Settings, default as default_settings};
use property_declaration_value::property_declaration_value_to_css_string;

trait ExtendFromPropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &mut Self;
}
impl ExtendFromPropertyDeclarationBlock for PropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &mut Self {
        for (declartion, importance) in block.declaration_importance_iter() {
            self.push(declartion.clone(), importance, DeclarationSource::Parsing);
        }

        self
    }
}

trait RemoveImportanceFromPropertyDeclarationBlock {
    fn remove_importance(self: &mut Self) -> &mut Self;
}
impl RemoveImportanceFromPropertyDeclarationBlock for PropertyDeclarationBlock {
    fn remove_importance(self: &mut Self) -> &mut Self {
        let property_declaration_block = self.clone();
        for property_declaration in property_declaration_block.declarations().iter() {
            let property_id = match property_declaration.id() {
                PropertyDeclarationId::Longhand(id) => PropertyId::Longhand(id),
                PropertyDeclarationId::Custom(name) => PropertyId::Custom(name.clone()),
            };
            self.set_importance(&property_id, Importance::Normal);
        }

        self
    }
}

trait RemoveExcludedPropertiesFromPropertyDeclarationBlock {
    fn remove_excluded_properties(self: &mut Self, properties: &Vec<&str>) -> &mut Self;
}
impl RemoveExcludedPropertiesFromPropertyDeclarationBlock for PropertyDeclarationBlock {
    fn remove_excluded_properties(self: &mut Self, properties: &Vec<&str>) -> &mut Self {
        for property_id in properties.iter() {
            match PropertyId::parse(property_id) {
                Ok(ref id) => {
                    self.remove_property(id);
                },
                _ => (),
            };
        }

        self
    }
}

#[derive(Clone, Debug)]
pub struct Eyeliner<'a> {
    pub document: NodeRef,
    pub stylesheet: Stylesheet,
    pub options: Options<'a>,
    pub settings: Settings<'a>,
    pub node_style_map: HashMap<HashableNodeRef, PropertyDeclarationBlock>,
    pub rules: Rules,
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
            node_style_map: HashMap::new(),
            rules: Rules::new(),
        }
    }
}

impl<'a> CollectRules for Eyeliner<'a> {
    fn collect_rules(self: &mut Self) -> &mut Self {
        {
            let read_guard = &self.stylesheet.shared_lock.read();
            for css_rule in &self.stylesheet.contents.rules.as_ref().read_with(read_guard).0 {
                match *css_rule {
                    CssRule::Style (ref style_rule_locked) => {
                        let style_rule = style_rule_locked.as_ref().read_with(read_guard);
                        let StyleRule { ref selectors, block: ref block_locked, .. } = *style_rule;

                        use servo_css_parser::cssparser::ToCss;
                        let mut block = block_locked.as_ref().read_with(read_guard).clone();
                        block.remove_excluded_properties(&self.settings.excluded_properties);
                        self.rules.style.push((
                            selectors.to_css_string(),
                            block,
                        ));
                    },

                    CssRule::Media (ref media_rule_locked) => {
                        if !self.options.preserve_media_queries {
                            continue;
                        }

                        let media_rule = media_rule_locked.as_ref().read_with(read_guard);

                        use servo_css_parser::style::shared_lock::ToCssWithGuard;
                        self.rules.media.push(media_rule.to_css_string(read_guard));
                    },

                    CssRule::FontFace (ref font_face_rule_data_locked) => {
                        if !self.options.preserve_font_faces {
                            continue;
                        }

                        let font_face_rule_data = font_face_rule_data_locked.as_ref().read_with(read_guard);

                        use servo_css_parser::style::shared_lock::ToCssWithGuard;
                        self.rules.font_face.push(font_face_rule_data.to_css_string(read_guard));
                    },

                    _ => (),
                }
            }
        }

        self
    }
}

impl<'a> ApplyRules for Eyeliner<'a> {
    fn apply_rules(self: &mut Self) -> &mut Self {
        for (selector, block) in self.rules.style.clone() {
            // TODO: using `::` seems to break things.
            // While testing using Bootstrap CSS, `::after` and `::before` give stack overflows.
            if selector.contains("::") {
                continue;
            }

            for node in self.document.select(&selector).unwrap() {
                if
                    self.settings.non_visual_elements.contains(
                        &node.name.local.chars().as_str().to_lowercase().as_str()
                    )
                {
                    continue;
                }

                match self.node_style_map.entry(HashableNodeRef::new(&node)) {
                    Occupied(mut entry) => {
                        entry.get_mut().extend_from_block(&block);
                    },
                    Vacant(entry) => {
                        let mut exisiting_style = parse_style_attribute(
                            &node.attributes.borrow_mut().get("style").unwrap_or(""),
                            &self.stylesheet.contents.url_data.read(),
                            &RustLogReporter {},
                            QuirksMode::NoQuirks,
                        );
                        exisiting_style.extend_from_block(&block);
                        entry.insert(exisiting_style.clone());
                    },
                };
            }
        }

        for (hash, block) in &self.node_style_map {
            let mut cloned_block = block.clone();
            if !self.options.preserve_important {
                cloned_block.remove_importance();
            }

            use servo_css_parser::style_traits::values::ToCss;
            hash.node.as_element().unwrap().attributes.borrow_mut().insert(
                "style",
                cloned_block.to_css_string(),
            );
        }

        self
    }
}

impl<'a> ApplyAttributes for Eyeliner<'a> {
    fn apply_attributes(self: &Self, property: &str) -> &Self {
        for (hash, block) in &self.node_style_map {
            let property_declaration = block.get(
                PropertyDeclarationId::Longhand(
                    PropertyId::parse(property).unwrap().longhand_id().unwrap()
                )
            );

            if property_declaration.is_none() {
                continue;
            }

            let element = hash.node.as_element().unwrap();

            let mut attributes = element.attributes.borrow_mut();
            use servo_css_parser::style_traits::values::ToCss;
            let value = property_declaration.unwrap().0.to_css_string();

            if value.ends_with("px") {
                attributes.insert(property, value.replace("px", ""));
                continue;
            }

            if
                value.ends_with("%") &&
                self.settings.table_elements.contains(
                    &element.name.local.chars().as_str().to_lowercase().as_str()
                )
            {
                attributes.insert(property, value);
            }
        }

        self
    }
}

impl<'a> ApplyWidthAttributes for Eyeliner<'a> {
    fn apply_width_attributes(self: &Self) -> &Self {
        if !self.options.apply_width_attributes {
            return self;
        }

        self.apply_attributes("width")
    }
}

impl<'a> ApplyHeightAttributes for Eyeliner<'a> {
    fn apply_height_attributes(self: &Self) -> &Self {
        if !self.options.apply_height_attributes {
            return self;
        }

        self.apply_attributes("height")
    }
}

impl<'a> ApplyTableElementAttributes for Eyeliner<'a> {
    fn apply_table_element_attributes(self: &Self) -> &Self {
        if !self.options.apply_table_element_attributes {
            return self;
        }

        for (hash, block) in &self.node_style_map {
            let element = hash.node.as_element().unwrap();

            if
                !self.settings.table_elements.contains(
                    &element.name.local.chars().as_str().to_lowercase().as_str()
                )
            {
                continue;
            }

            let mut attributes = element.attributes.borrow_mut();

            for property_declaration in block.declarations() {
                let property = property_declaration.id().name().to_string();
                let attribute = self.settings.style_to_attribute.get::<str>(&property);

                if attribute.is_none() {
                    continue;
                }

                attributes.insert(
                    attribute.unwrap().clone(),
                    property_declaration_value_to_css_string(property_declaration)
                );
            }
        }

        self
    }
}

impl<'a> InsertPreservedCss for Eyeliner<'a> {
    fn insert_preserved_css(self: &Self) -> &Self {
        for node_to_insert_style_into in &self.options.insert_preserved_css {
            let nodes = self.document.select(node_to_insert_style_into);

            if !nodes.is_ok() {
                continue;
            }

            for node in nodes.unwrap() {
                let mut preserved_css = vec![];
                preserved_css.extend_from_slice(&self.rules.font_face);
                preserved_css.extend_from_slice(&self.rules.media);

                let text_node = NodeRef::new_text(preserved_css.join("\n"));
                let style_node = NodeRef::new_element(
                    QualName { prefix: None, ns: ns!(), local: local_name!("style") },
                    vec![]
                );
                style_node.append(text_node);
                node.as_node().append(style_node);
                return self;
            }
        }

        self
    }
}

impl<'a> ToString for Eyeliner<'a> {
    fn to_string(self: &Self) -> String {
        self.document.to_string()
    }
}
