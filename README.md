# mdbook-playscript

An mdBook preprocessor for writing stage play scripts.
The library crate of this preprocessor is [mdplayscript](https://crates.io/crates/mdplayscript).

If you are a Japanese user, read [Japanese README](./README_ja.md), would you?

## License

License is changed in the version 0.3.0.

The source codes written in Rust and CSS files are licensed under MPL-2.0.

Markdown files in [`examples/figaro/`](examples/figaro/) are licensed under CC BY-SA 3.0, because they are copied from [WikiSource](https://fr.wikisource.org/wiki/Le_Mariage_de_Figaro) and formatted by the author of this crate.

Markdown files in [`examples/torikaeshi/`](examples/torikaeshi) are in public domain, because they are copied from [Aozora Bunko (in Japanese)](https://www.aozora.gr.jp/cards/000076/card46934.html) and formatted by the author of this crate.

## Install

You can install this preprocessor with cargo:
```
cargo install mdbook-playscript
```

Other installation methods are not provided now.

## Usage

You can use `mdbook-playscript` preprocessor by adding the following config to your `book.toml`.
You must specify the `additional-css` for the HTML backend by hand.

```toml
[preprocessor.playscript]
command = "mdbook-playscript"

[output.html]
additional-css = ["mdplayscript.css"]
```

## Examples

The [README of mdplayscript](https://crates.io/crates/mdplayscript) explains basic notations.

An example of a source of a book preprocessed with `mdbook-playscript` is placed in [examples/figaro](examples/figaro).
The generated example is hosted on [github pages](https://shotarotsuji.github.io/mdbook-playscript/figaro/).
