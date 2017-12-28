#![feature(test)]

extern crate test;
extern crate eyeliner;

use test::Bencher;
use eyeliner::inline;

#[bench]
fn bench_dashboard(b: &mut Bencher) {
    let html = include_str!("./bootstrap/docs/4.0/examples/dashboard/index.html");
    let css = include_str!("./bootstrap/dist/css/bootstrap.min.css");
    println!("{}", inline(html, css));
    b.iter(|| inline(html, css))
}

#[bench]
fn bench_navbar(b: &mut Bencher) {
    let html = include_str!("./bootstrap/docs/4.0/examples/navbars/index.html");
    let css = include_str!("./bootstrap/dist/css/bootstrap.min.css");
    b.iter(|| inline(html, css))
}
