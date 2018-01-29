pub mod default {
    /// Options for ways to modify the HTML document using CSS.
    #[derive(Clone, Debug)]
    pub struct Options {
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

        // TODO: pub web_resources: Option<OptionsForHttpClientToInlineRemoteResources>,
    }

    impl Default for Options {
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
}

/// The required options to inline HTML and CSS. Use the other `Options`, it is has nice defaults.
#[derive(Clone, Debug)]
pub struct Options {
    pub apply_table_element_attributes: bool,
    pub apply_height_attributes: bool,
    pub apply_style_tags: bool,
    pub apply_width_attributes: bool,
    pub insert_preserved_css: Vec<String>,
    pub preserve_font_faces: bool,
    pub preserve_important: bool,
    pub preserve_media_queries: bool,
    pub remove_style_tags: bool,
    // TODO: pub web_resources: OptionsForHttpClientToInlineRemoteResources,
}

impl Options {
    /// Takes the other `Options` and uses any fields set on it or defaults to another value.
    pub fn new(opt: default::Options) -> Self {
        Self {
            apply_table_element_attributes: opt.apply_table_element_attributes.unwrap_or(true),
            apply_height_attributes: opt.apply_height_attributes.unwrap_or(true),
            apply_style_tags: opt.apply_style_tags.unwrap_or(true),
            apply_width_attributes: opt.apply_width_attributes.unwrap_or(true),
            insert_preserved_css: opt.insert_preserved_css.unwrap_or_else(
                || vec!["head".to_owned(), "body".to_owned(), "html".to_owned()]
            ),
            preserve_font_faces: opt.preserve_font_faces.unwrap_or(true),
            preserve_important: opt.preserve_important.unwrap_or(false),
            preserve_media_queries: opt.preserve_media_queries.unwrap_or(true),
            remove_style_tags: opt.remove_style_tags.unwrap_or(true),
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Self::new(default::Options::default())
    }
}
