extern crate eyeliner;
extern crate kuchiki;

use eyeliner::{inline, Options, Settings};
use kuchiki::traits::*;
use kuchiki::parse_html;

#[test]
fn test() {
    let expected_document = parse_html().one(include_str!("./fixture.html"));
    let result_document = parse_html().one(
        inline(
            include_str!("./test.html"),
            Some(include_str!("./test.css").to_owned()),
            None,
            None,
        )
    );

    println!("{}", inline(
        include_str!("./test.html"),
        Some(include_str!("./test.css").to_owned()),
        None,
        None,
    ));

    let selector = "#test1, #test2";
    let expected_select = expected_document.select(selector).unwrap();
    let result_select = result_document.select(selector).unwrap();

    for (expected_node, result_node) in expected_select.zip(result_select) {
        let expected_attributes = expected_node.attributes.borrow();
        let result_attributes = result_node.attributes.borrow();

        let attribute = "style";
        let expected_style = expected_attributes.get(attribute).unwrap();
        let result_style = result_attributes.get(attribute).unwrap();

        println!("\nExpected:\t{}\nResult:  \t{}", expected_style, result_style);

        assert_eq!(result_style, expected_style);
    }
}
