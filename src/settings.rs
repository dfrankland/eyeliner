use maplit::hashmap;
use std::collections::HashMap;

/// Settings referenced by features enabled through `Options`.
#[derive(Clone, Debug)]
pub struct AbstractSettings {
    /// List of HTML elements that can receive `width` attributes.
    ///
    /// Defaults to:
    ///
    /// ```
    /// vec![
    ///     "table",
    ///     "td",
    ///     "img",
    /// ];
    /// ```
    ///
    pub width_elements: Option<Vec<String>>,

    /// List of HTML elements that can receive `height` attributes.
    ///
    /// Defaults to:
    ///
    /// ```
    /// vec![
    ///     "table",
    ///     "td",
    ///     "img",
    /// ];
    /// ```
    ///
    pub height_elements: Option<Vec<String>>,

    /// Map of style property names to their respective attribute names.
    ///
    /// Defaults to:
    ///
    /// ```
    /// # #[macro_use] extern crate maplit;
    /// # fn main() {
    /// hashmap!{
    ///     "background-color" => "bgcolor",
    ///     "background-image" => "background",
    ///     "text-align" => "align",
    ///     "vertical-align" => "valign",
    /// };
    /// # }
    /// ```
    ///
    pub style_to_attribute: Option<HashMap<String, String>>,

    /// List of table HTML elements that can receive attributes defined in
    /// `Settings.style_to_attribute`.
    ///
    /// Defaults to:
    ///
    /// ```
    /// vec![
    ///     "table",
    ///     "td",
    ///     "th",
    ///     "tr",
    ///     "td",
    ///     "caption",
    ///     "colgroup",
    ///     "col",
    ///     "thead",
    ///     "tbody",
    ///     "tfoot",
    /// ];
    /// ```
    ///
    pub table_elements: Option<Vec<String>>,

    /// List of elements that will not have styles inlined because they are not intended to
    /// render.
    ///
    /// Defaults to:
    ///
    /// ```
    /// vec![
    ///     "head",
    ///     "title",
    ///     "base",
    ///     "link",
    ///     "style",
    ///     "meta",
    ///     "script",
    ///     "noscript",
    /// ];
    /// ```
    ///
    pub non_visual_elements: Option<Vec<String>>,

    /// List of CSS style properties that will not be inlined.
    ///
    /// Defaults to:
    ///
    /// ```
    /// Vec::<&str>::new();
    /// ```
    ///
    pub excluded_properties: Option<Vec<String>>,
}

impl Default for AbstractSettings {
    fn default() -> Self {
        Self {
            width_elements: None,
            height_elements: None,
            style_to_attribute: None,
            table_elements: None,
            non_visual_elements: None,
            excluded_properties: None,
        }
    }
}

impl From<ConcreteSettings> for AbstractSettings {
    fn from(concrete_settings: ConcreteSettings) -> Self {
        Self {
            width_elements: Some(concrete_settings.width_elements),
            height_elements: Some(concrete_settings.height_elements),
            style_to_attribute: Some(concrete_settings.style_to_attribute),
            table_elements: Some(concrete_settings.table_elements),
            non_visual_elements: Some(concrete_settings.non_visual_elements),
            excluded_properties: Some(concrete_settings.excluded_properties),
        }
    }
}

/// The required settings to inline HTML and CSS. Use the other `Settings`, it is has nice
/// defaults.
#[derive(Clone, Debug)]
pub struct ConcreteSettings {
    pub width_elements: Vec<String>,
    pub height_elements: Vec<String>,
    pub style_to_attribute: HashMap<String, String>,
    pub table_elements: Vec<String>,
    pub non_visual_elements: Vec<String>,
    pub excluded_properties: Vec<String>,
}

impl From<AbstractSettings> for ConcreteSettings {
    /// Takes the other `Settings` and uses any fields set on it or defaults to another value.
    fn from(abstract_settings: AbstractSettings) -> Self {
        Self {
            width_elements: abstract_settings.width_elements.unwrap_or_else(|| {
                vec!["table", "td", "img"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            }),
            height_elements: abstract_settings.height_elements.unwrap_or_else(|| {
                vec!["table", "td", "img"]
                    .iter()
                    .map(|x| x.to_string())
                    .collect()
            }),
            table_elements: abstract_settings.table_elements.unwrap_or_else(|| {
                vec![
                    "table", "td", "th", "tr", "td", "caption", "colgroup", "col", "thead",
                    "tbody", "tfoot",
                ]
                .iter()
                .map(|x| x.to_string())
                .collect()
            }),
            style_to_attribute: abstract_settings.style_to_attribute.unwrap_or_else(|| {
                (hashmap! {
                    "background-color" => "bgcolor",
                    "background-image" => "background",
                    "text-align" => "align",
                    "vertical-align" => "valign",
                })
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect()
            }),
            non_visual_elements: abstract_settings.non_visual_elements.unwrap_or_else(|| {
                vec![
                    "head", "title", "base", "link", "style", "meta", "script", "noscript",
                ]
                .iter()
                .map(|x| x.to_string())
                .collect()
            }),
            excluded_properties: abstract_settings
                .excluded_properties
                .unwrap_or_else(|| vec![]),
        }
    }
}

impl Default for ConcreteSettings {
    fn default() -> Self {
        Self::from(AbstractSettings::default())
    }
}
