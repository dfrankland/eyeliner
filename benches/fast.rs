#![feature(test)]

extern crate test;
extern crate eyeliner;

use test::Bencher;
use eyeliner::inline;

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
