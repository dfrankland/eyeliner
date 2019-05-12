#![feature(test)]

extern crate test;

use eyeliner::inline;
use servo_css_parser::embedder_traits;
use test::Bencher;

#[bench]
fn bench_dashboard(b: &mut Bencher) {
    embedder_traits::resources::set_for_tests();
    let html = include_str!("./bootstrap/site/content/docs/4.3/examples/dashboard/index.html");
    let css = include_str!("./bootstrap/dist/css/bootstrap.css");
    b.iter(|| inline(html, Some(css.to_owned()), None, None))
}

#[bench]
fn bench_navbar(b: &mut Bencher) {
    embedder_traits::resources::set_for_tests();
    let html = include_str!("./bootstrap/site/content/docs/4.3/examples/navbars/index.html");
    let css = include_str!("./bootstrap/dist/css/bootstrap.css");
    b.iter(|| inline(html, Some(css.to_owned()), None, None))
}
