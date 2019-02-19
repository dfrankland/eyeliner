use std::{
    collections::{
        HashMap,
        hash_map::Entry::{Occupied, Vacant}
    },
    string::ToString
};
use kuchiki::{
    traits::*,
    parse_html,
    NodeRef,
};
use html5ever::{QualName, ns, local_name, namespace_url};
use servo_css_parser::{
    parse,
    types::{Url, QuirksMode, MediaList, Origin, ServoStylesheet as Stylesheet},
    style::{
        stylesheets::{CssRule, StyleRule},
        properties::{
            declaration_block::{parse_style_attribute, PropertyDeclarationBlock, Importance},
            PropertyDeclarationId, PropertyId,
        },
    },
};
use super::{
    traits::*,
    hash::HashableNodeRef,
    rules::Rules,
    options::ConcreteOptions,
    settings::ConcreteSettings,
};

trait ExtendFromPropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &mut Self;
}
impl ExtendFromPropertyDeclarationBlock for PropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &mut Self {
        for (declartion, importance) in block.declaration_importance_iter() {
            self.push(declartion.clone(), importance);
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
            self.push(property_declaration.clone(), Importance::Normal);
        }

        self
    }
}

trait RemoveExcludedPropertiesFromPropertyDeclarationBlock {
    fn remove_excluded_properties(self: &mut Self, properties: &[String]) -> &mut Self;
}
impl RemoveExcludedPropertiesFromPropertyDeclarationBlock for PropertyDeclarationBlock {
    fn remove_excluded_properties(self: &mut Self, properties: &[String]) -> &mut Self {
        for property_id in properties {
            if let Ok(ref id) = PropertyId::parse_enabled_for_all_content(property_id) {
                if let Some(first_declaration_to_remove) = self.first_declaration_to_remove(id) {
                    self.remove_property(id, first_declaration_to_remove);
                }
            }
        }

        self
    }
}

/// Data and methods related to modifying HTML with CSS.
#[derive(Clone, Debug)]
pub struct Eyeliner {
    /// A strong reference to the root node of the HTML document.
    pub document: NodeRef,

    /// A structure representing the CSS stylesheet.
    pub stylesheet: Stylesheet,

    /// Options for ways to modify the HTML document using CSS.
    pub options: ConcreteOptions,

    /// Settings referenced by features enabled through options.
    pub settings: ConcreteSettings,

    /// A hashmap of HTML elements to CSS style.
    pub node_style_map: HashMap<HashableNodeRef, PropertyDeclarationBlock>,

    /// Data collected from the CSS stylesheet.
    pub rules: Rules,
}

impl Eyeliner {
    /// Create a new instance to inline HTML with CSS, using concreate options and settings.
    ///
    /// 1.  Opitionally extracts the CSS in `<style />` tags from the HTML document. Then,
    ///     optionally removes the `<style />` tag from the HTML document.
    ///
    /// 2.  Any CSS extraced gets appended to the `css` argument.
    ///
    pub fn new<T: Into<ConcreteOptions>, U: Into<ConcreteSettings>>(
        html: &str,
        css: Option<String>,
        options: Option<T>,
        settings: Option<U>,
    ) -> Self {
        let options = match options {
            Some(o) => o.into(),
            None => ConcreteOptions::default(),
        };
        let settings = match settings {
            Some(s) => s.into(),
            None => ConcreteSettings::default(),
        };

        let mut css = css.unwrap_or_else(String::new);
        let document = parse_html().one(html);

        if options.apply_style_tags {
            if let Ok(nodes) = document.select("style") {
                for node in nodes {
                    css += &node.text_contents();
                    if options.remove_style_tags {
                        node.as_node().detach();
                    }
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
            rules: Rules::default(),
        }
    }
}

impl CollectRules for Eyeliner {
    /// Collects CSS rules from the CSS stylesheet for other methods to use.
    /// Optionally removes any excluded CSS properties.
    /// Optionally preserves `@media` and `@font-face` rules.
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

impl ApplyRules for Eyeliner {
    /// Inlines the CSS rules extracted from the CSS stylesheet into the HTML document.
    ///
    /// 1.  For each CSS rule selector (excluding pseudo-selectors), find the matching nodes in the
    ///     HTML document. Skips any non-visual elements.
    ///
    /// 2.  Each elements style is hashmapped. If any element has a `style` attribute is is
    ///     extended by each of the CSS rules that apply to it. Optionally preserves `!important`.
    ///
    fn apply_rules(self: &mut Self) -> &mut Self {
        for (selector, block) in self.rules.style.clone() {
            // TODO: using `::` seems to break things.
            // While testing using Bootstrap CSS, `::after` and `::before` give stack overflows.
            if selector.contains("::") {
                continue;
            }

            let nodes = match self.document.select(&selector) {
                Ok(n) => n,
                _ => continue,
            };

            for node in nodes {
                if
                    self.settings.non_visual_elements.contains(
                        &node.name.local.chars().as_str().to_lowercase()
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
                            node.attributes.borrow_mut().get("style").unwrap_or(""),
                            &self.stylesheet.contents.url_data.read(),
                            None,
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

            if let Some(element) = hash.node.as_element() {
                let mut css = String::default();
                cloned_block.to_css(&mut css).unwrap();
                element.attributes.borrow_mut().insert(
                    "style",
                    css,
                );
            }
        }

        self
    }
}

impl ApplyAttributes for Eyeliner {
    /// Iterates over all elements and applies a matching attribute if it has the given CSS
    /// property.
    fn apply_attributes(self: &Self, property: &str) -> &Self {
        for (hash, block) in &self.node_style_map {
            let property_declaration = block.get(
                PropertyDeclarationId::Longhand(
                    PropertyId::parse_enabled_for_all_content(property).unwrap().longhand_id().unwrap()
                )
            );

            if property_declaration.is_none() {
                continue;
            }

            let element = hash.node.as_element().unwrap();

            let mut attributes = element.attributes.borrow_mut();
            let mut value = String::default();
            property_declaration.unwrap().0.to_css(&mut value).unwrap();

            if value.ends_with("px") {
                attributes.insert(property, value.replace("px", ""));
                continue;
            }

            if
                value.ends_with('%') &&
                self.settings.table_elements.contains(
                    &element.name.local.chars().as_str().to_lowercase()
                )
            {
                attributes.insert(property, value);
            }
        }

        self
    }
}

impl ApplyWidthAttributes for Eyeliner {
    /// Optionally iterates over all elements and applies a `width` attribute if it has a `width`
    /// CSS property applied to it.
    fn apply_width_attributes(self: &Self) -> &Self {
        if !self.options.apply_width_attributes {
            return self;
        }

        self.apply_attributes("width")
    }
}

impl ApplyHeightAttributes for Eyeliner {
    /// Optionally iterates over all elements and applies a `height` attribute if it has a `height`
    /// CSS property applied to it.
    fn apply_height_attributes(self: &Self) -> &Self {
        if !self.options.apply_height_attributes {
            return self;
        }

        self.apply_attributes("height")
    }
}

#[cfg_attr(feature = "cargo-clippy", allow(clone_double_ref))]
impl ApplyTableElementAttributes for Eyeliner {
    /// Applies attributes to table elements.
    ///
    /// 1.  Iterates over all elements and matches those that are specified table elements.
    ///
    /// 2.  If elements have style properties that are mapped to attributes, then their mapped
    ///     attributes are applied.
    ///
    fn apply_table_element_attributes(self: &Self) -> &Self {
        if !self.options.apply_table_element_attributes {
            return self;
        }

        for (hash, block) in &self.node_style_map {
            let element = hash.node.as_element().unwrap();

            if
                !self.settings.table_elements.contains(
                    &element.name.local.chars().as_str().to_lowercase()
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

                let mut css = String::default();
                property_declaration.to_css(&mut css).unwrap();

                attributes.insert(
                    attribute.unwrap().clone(),
                    css,
                );
            }
        }

        self
    }
}

impl InsertPreservedCss for Eyeliner {
    /// Tries to insert any `@media` or `@font-face` rules collected into the locations specified
    // in the HTML document.
    fn insert_preserved_css(self: &Self) -> &Self {
        for node_to_insert_style_into in &self.options.insert_preserved_css {
            let mut nodes = match self.document.select(node_to_insert_style_into) {
                Ok(n) => n,
                _ => continue,
            };

            let node = match nodes.next() {
                Some(n) => n,
                None => continue,
            };

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

        self
    }
}

impl ToString for Eyeliner {
    fn to_string(self: &Self) -> String {
        self.document.to_string()
    }
}
