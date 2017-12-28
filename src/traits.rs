use servo_css_parser::style::properties::declaration_block::PropertyDeclarationBlock;
use servo_css_parser::style::stylesheets::CssRules;
use servo_css_parser::style::properties::BuilderArc as Arc;
use servo_css_parser::style::shared_lock::Locked;

use rules::EyelinerRules;

pub trait StylesheetAsEyelinerRules {
    fn stylesheet_as_eyeliner_rules(self: &Self, rules: &Arc<Locked<CssRules>>) -> EyelinerRules;
}

pub trait ExtendFromPropertyDeclarationBlock {
    fn extend_from_block(self: &mut Self, block: &PropertyDeclarationBlock) -> &Self;
}

pub trait InlineStylesheetAndDocument {
    fn inline_stylesheet_and_document(self: &Self) -> &Self;
}

pub trait SerializeDocument {
    fn serialize_document(self: &Self) -> String;
}
