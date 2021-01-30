# mdbook-playscript

A mdBook preprocessor for writing play scripts.
The library crate for this preprocessor is [mdplayscript](https://crates.io/crates/mdplayscript).

For Japanese users, there is [Japanese README](./README_ja.md).

## Install

This preprocessor can be installed with cargo:

```
cargo install mdbook-playscript
```

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

The README of [mdplayscript](https://crates.io/crates/mdplayscript) explains basic notations.

An example of a source of mdBook with `mdbook-playscript` is placed in [examples/figaro](examples/figaro).
The generated example is hosted on [github pages](https://shotarotsuji.github.io/mdbook-playscript/figaro/).
