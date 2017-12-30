# eyeliner

A CSS inliner for making emails.

## Purpose

Email is still one of the most painful things that developers have to work with.
Even though most browsers have advanced and brought amazing features to the web,
email remains one of the last bastions of awful content to develop. Styling
emails is at the top of the list of annoyances, all of it must be written inline
making it impossible to maintain.

There are a few popular tools that do this:

*   [Juice][juice] (Node.js), powered by [Cheerio][cheerio].

[juice]: https://github.com/Automattic/juice
[cheerio]: https://github.com/cheeriojs/cheerio

*   [Premailer][premailer] / [Roadie][roadie] (Ruby), powered by
    [Nokogiri][nokogiri].

[premailer]: https://github.com/premailer/premailer
[roadie]: https://github.com/Mange/roadie
[nokogiri]: https://github.com/sparklemotion/nokogiri

*   [CssToInlineStyles][CssToInlineStyles] (PHP), powered by
    [Symfony's `css-selector`][css-selector].

[CssToInlineStyles]: https://github.com/tijsverkoyen/CssToInlineStyles
[css-selector]: https://github.com/symfony/css-selector

These tools are mainly used as a pre-process for making email templates due to
the overhead caused by inlining, but this can be an issue for projects that need
to process many dynamic emails at scale. That is the reason why `eyeliner` is so
useful. Rust and [Servo][servo], the framework that powers `eyeliner`, have
amazing performance allowing emails to be inlined within mere milliseconds
unlocking the potential for processing emails and sending them on-the-fly.

## How to use

[Documentation][documentation]

[documentation]: https://dfrankland.github.io/eyeliner/

**Much of [Servo][servo] and other dependencies that `eyeliner` uses are under
development, use `eyeliner` with caution.**

>   Because `eyeliner` depends on [Servo][servo], it requires the same build
>   prerequisites and installation instructions [documented here][servo docs].
>
>   For example, on OSX:
>
>   ```sh
>   brew install automake pkg-config python cmake yasm
>   pip install virtualenv
>   ```

[servo docs]: https://github.com/servo/servo/blob/master/README.md#setting-up-your-environment

1.  Add this repo to your `Cargo.toml` file:

    ```toml
    [dependencies]
    eyeliner = { git = "https://github.com/dfrankland/eyeliner.git" }
    ```

2.  Pass your HTML and CSS to the `inline` function to get a new string of HTML
    with all of the CSS inlined.

    ```rust
    extern crate eyeliner;

    use eyeliner::inline;

    fn main() {
      let html = r#"
        <!doctype html>
        <html>
          <head>
            <title>Test</title>
          </head>
          <body>
            <h1>Hello, world!</h1>
            <p>I <span class="red">love</span> Rust!</p>
          </body>
        </html>
      "#;

      let css = r#"
        .red {
          color: red;
        }
      "#;

      let inlined_html = inline(html, css);

      println!("{}", inlined_html);

      // Prints out the following:
      //
      // <!doctype html>
      // <html>
      //   <head>
      //     <title>Test</title>
      //   </head>
      //   <body>
      //     <h1>Hello, world!</h1>
      //     <p>I <span class="red" style="color: red;">love</span> Rust!</p>
      //   </body>
      // </html>
    }
    ```

[servo]: https://github.com/servo/servo
