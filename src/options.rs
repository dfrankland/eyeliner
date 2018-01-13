pub mod default {
    pub struct Options<'a> {
        pub apply_attributes_table_elements: Option<bool>,
        pub apply_height_attributes: Option<bool>,
        pub apply_style_tags: Option<bool>,
        pub apply_width_attributes: Option<bool>,
        pub insert_preserved_css: Option<&'a str>,
        pub preserve_font_faces: Option<bool>,
        pub preserve_important: Option<bool>,
        pub preserve_media_queries: Option<bool>,
        pub remove_style_tags: Option<bool>,
        // TODO: pub web_resources: Option<OptionsForHttpClientToInlineRemoteResources>,
    }

    impl<'a> Default for Options<'a> {
        fn default() -> Self {
            Self {
                apply_attributes_table_elements: None,
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

pub struct Options<'a> {
    pub apply_attributes_table_elements: bool,
    pub apply_height_attributes: bool,
    pub apply_style_tags: bool,
    pub apply_width_attributes: bool,
    pub insert_preserved_css: &'a str,
    pub preserve_font_faces: bool,
    pub preserve_important: bool,
    pub preserve_media_queries: bool,
    pub remove_style_tags: bool,
    // TODO: pub web_resources: OptionsForHttpClientToInlineRemoteResources,
}

impl<'a> Options<'a> {
    pub fn new(opt: default::Options<'a>) -> Self {
        Self {
            apply_attributes_table_elements: opt.apply_attributes_table_elements.unwrap_or(true),
            apply_height_attributes: opt.apply_height_attributes.unwrap_or(true),
            apply_style_tags: opt.apply_style_tags.unwrap_or(true),
            apply_width_attributes: opt.apply_width_attributes.unwrap_or(true),
            insert_preserved_css: opt.insert_preserved_css.unwrap_or("head, body, html"),
            preserve_font_faces: opt.preserve_font_faces.unwrap_or(true),
            preserve_important: opt.preserve_important.unwrap_or(false),
            preserve_media_queries: opt.preserve_media_queries.unwrap_or(true),
            remove_style_tags: opt.remove_style_tags.unwrap_or(true),
        }
    }
}
