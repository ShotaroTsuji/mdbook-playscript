# mdbook-playscript

戯曲を記述するために記法を拡張したMarkdownを[mdBook](https://github.com/rust-lang/mdBook)で利用するためのプリプロセッサです。

## インストール

現在はcargoでのインストールのみが可能です。

```
cargo install mdbook-playscript
```

## 使い方

`mdbook init`を実行した後に`book.toml`に以下の記述を追加してください。
その後、`mdbook build`を実行すると拡張した記法を解釈した上でスタイル付けがなされたHTMLが出力されます。

日本語で戯曲を書く場合は、キー`book.language`の値を`ja`にしてください。
`mdbook-playscript`は`book.language`の値を元にコピーするCSSファイルを決定します。
`book.language = "ja"`のときは、`mdbook init`を実行したディレクトリに`mdplayscript_ja.css`をコピーします。

また、`output.html.additional-css`の値を`mdplayscript_ja.css`にする必要があります。
プリプロセッサがコンフィグを書き換えることはできないので手動で設定しなければなりません。

```toml
[book]
authors = ["（著者の名前）"]
language = "ja"
multilingual = false
src = "src"
title = "（作品のタイトル）"

[preprocessor.playscript]
command = "mdbook-playscript"

[output.html]
additional-css = ["mdplayscript_ja.css"]
```

## 記法

### 台詞

戯曲の台詞を書くために拡張した記法が導入されています。

記号`>`を使って登場人物とその台詞を記述することができます。

```
人物名> 台詞
```

ただし、`>`が行頭にある場合はMarkdownのブロッククオートとして解釈されます。

また、半角の括弧`()`を使うことでト書きを記述することができます。

```
人物名> 台詞(ト書き)
```

台詞には改行を含むことができます。ただし、空行は台詞の終わりと解釈されます。

```
人物名> 台詞1
台詞2

この行は台詞とはみなされない。
```

記号`>`を含まない段落でも次のHTMLコメントを記述することで台詞としてフォーマットさせることができます。
monologueという文字列が含まれますが、日本語戯曲では独白のみの場面であっても登場人物名を見出しに立てるようなので、この記法はト書きを独立させたいとき以外には使わないでしょう。

```
<!-- playscript-monologue-begin -->
(ト書き)
<!-- playscript-monologue-end -->
```

### その他の記法

`<!-- playscript-off -->`で`mdplayscript`による変換をしないように指示できます。

`<!-- playscript-on -->`で`mdplayscript`による変換を行うように指示できます。

`<!-- playscript-title -->`で`book.toml`に記述したタイトルを出力できます。

`<!-- playscript-authors -->`で`book.toml`に記述したタイトルを出力できます。

## 実例

[岡本かの子「取返し物語」](https://shotarotsuji.github.io/mdbook-playscript/torikaeshi/)を`mdbook-playscript`を使って整形した戯曲の例として公開しています。
ソースコードは[examples/torikaeshi](./examples/torikaeshi)にあります。
なお、この戯曲は青空文庫より複製・整形したものです。
