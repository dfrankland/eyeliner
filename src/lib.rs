#![feature(test)]

extern crate kuchiki;
extern crate servo_css_parser;
#[macro_use] extern crate markup5ever;
extern crate test;

pub mod eyeliner;

use eyeliner::{Eyeliner, InlineStylesheetAndDocument};

use test::Bencher;

pub fn inline<'a>(html: &'a str, css: &'a str) -> String {
    let eyeliner = Eyeliner::new(html, css);
    eyeliner.inline_stylesheet_and_document()
}

#[test]
fn test() {
    let html = r#"
        <html>
            <head>
                <title>Hello, world!</title>
            </head>
            <body>
                <h1>Hello, world!</h1>
                <p class="foo bar">I love HTML</p>
                <heart>&lt;3</heart>
            </body>
        </html>
    "#;

    let css = r#"
        .foo {
            color: black;
        }
        .foo.bar, heart {
            color: red;
            font-weight: bold;
        }
        .foo.bar {
            text-decoration: underline !important;
        }
        .foo.bar {
            text-decoration: inherit;
        }
    "#;

    let expected_result = r#"
        <html>
            <head>
                <title>Hello, world!</title>
            </head>
            <body>
                <h1>Hello, world!</h1>
                <p style="color: red; font-weight: bold; text-decoration: underline !important;">I love HTML</p>
                <heart style="color: red; font-weight: bold;">&lt;3</heart>
            </body>
        </html>
    "#;

    let result: String = inline(html, css);

    let expected_result_vec: Vec<&str> = expected_result.split_whitespace().collect();
    let result_vec: Vec<&str> = result.split_whitespace().collect();

    assert_eq!(
        expected_result_vec.join(""),
        result_vec.join(""),
    );
}

#[bench]
fn bench(b: &mut Bencher) {
    let html = r#"
        <html>
            <head>
                <title>Hello, world!</title>
            </head>
            <body>
                <h1>Hello, world!</h1>
                <p class="foo bar">I love HTML</p>
                <heart>&lt;3</heart>
            </body>
        </html>
    "#;

    let css = r#"
        .foo {
            color: black;
        }
        .foo.bar, heart {
            color: red;
            font-weight: bold;
        }
        .foo.bar {
            text-decoration: underline !important;
        }
        .foo.bar {
            text-decoration: inherit;
        }
    "#;

    b.iter(|| {
        test::black_box(inline(html, css));
    });
}
