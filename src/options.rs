/// Options for ways to modify the HTML document using CSS.
#[derive(Clone, Debug)]
pub struct AbstractOptions {
    /// Whether attributes specified in `Settings.style_to_attribute` get applied to table
    /// elements in `Settings.table_elements`.
    /// Defaults to `true`.
    pub apply_table_element_attributes: Option<bool>,

    /// Whether to use any CSS pixel heights to create `height` attributes on elements set in
    /// `Settings.height_elements`.
    /// Defaults to `true`.
    pub apply_height_attributes: Option<bool>,

    /// Whether to inline CSS in `<style />` tags in the HTML document.
    /// Defaults to `true`.
    pub apply_style_tags: Option<bool>,

    /// Whether to use any CSS pixel widths to create `width` attributes on elements set in
    /// `Settings.width_elements`.
    /// Defaults to `true`.
    pub apply_width_attributes: Option<bool>,

    /// List of elements to try to inline preserved `@media` and `@font-face` CSS rules into.
    /// Give an empty list to prevent inlining preserved CSS.
    /// Defaults to `["head", "body", "html"]`.
    pub insert_preserved_css: Option<Vec<String>>,

    /// Whether to preserve `@font-face` CSS rules.
    /// Defaults to `true`.
    pub preserve_font_faces: Option<bool>,

    /// Whether to preserve `!important` in CSS rules.
    /// Defaults to `false`.
    pub preserve_important: Option<bool>,

    /// Whether to preserve `@media` CSS rules.
    /// Defaults to `true`.
    pub preserve_media_queries: Option<bool>,

    /// Whether to remove `<style />` tags, after they have optionally had their CSS extracted.
    /// Defaults to `true`.
    pub remove_style_tags: Option<bool>,
}

impl Default for AbstractOptions {
    fn default() -> Self {
        Self {
            apply_table_element_attributes: None,
            apply_height_attributes: None,
            apply_style_tags: None,
            apply_width_attributes: None,
            insert_preserved_css: None,
            preserve_font_faces: None,
            preserve_important: None,
            preserve_media_queries: None,
            remove_style_tags: None,
        }
    }
}

impl From<ConcreteOptions> for AbstractOptions {
    fn from(concrete_options: ConcreteOptions) -> Self {
        AbstractOptions {
            apply_table_element_attributes: Some(concrete_options.apply_table_element_attributes),
            apply_height_attributes: Some(concrete_options.apply_height_attributes),
            apply_style_tags: Some(concrete_options.apply_style_tags),
            apply_width_attributes: Some(concrete_options.apply_width_attributes),
            insert_preserved_css: Some(concrete_options.insert_preserved_css),
            preserve_font_faces: Some(concrete_options.preserve_font_faces),
            preserve_important: Some(concrete_options.preserve_important),
            preserve_media_queries: Some(concrete_options.preserve_media_queries),
            remove_style_tags: Some(concrete_options.remove_style_tags),
        }
    }
}

/// The required options to inline HTML and CSS. Use the other `Options`, it is has nice defaults.
#[derive(Clone, Debug)]
pub struct ConcreteOptions {
    pub apply_table_element_attributes: bool,
    pub apply_height_attributes: bool,
    pub apply_style_tags: bool,
    pub apply_width_attributes: bool,
    pub insert_preserved_css: Vec<String>,
    pub preserve_font_faces: bool,
    pub preserve_important: bool,
    pub preserve_media_queries: bool,
    pub remove_style_tags: bool,
}

impl Default for ConcreteOptions {
    fn default() -> Self {
        Self::from(AbstractOptions::default())
    }
}

impl From<AbstractOptions> for ConcreteOptions {
    fn from(abstract_options: AbstractOptions) -> Self {
        Self {
            apply_table_element_attributes: abstract_options
                .apply_table_element_attributes
                .unwrap_or(true),
            apply_height_attributes: abstract_options.apply_height_attributes.unwrap_or(true),
            apply_style_tags: abstract_options.apply_style_tags.unwrap_or(true),
            apply_width_attributes: abstract_options.apply_width_attributes.unwrap_or(true),
            insert_preserved_css: abstract_options.insert_preserved_css.unwrap_or_else(|| {
                vec!["head", "body", "html"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            }),
            preserve_font_faces: abstract_options.preserve_font_faces.unwrap_or(true),
            preserve_important: abstract_options.preserve_important.unwrap_or(false),
            preserve_media_queries: abstract_options.preserve_media_queries.unwrap_or(true),
            remove_style_tags: abstract_options.remove_style_tags.unwrap_or(true),
        }
    }
}
