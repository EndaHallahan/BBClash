# BBClash

A robust, opinionated, performance-focused BBCode to HTML parser and compiler.

## What is BBClash?

BBClash is the open-source version of the BBCode compiler being built for [Penclash](https://endahallahan.github.io/Penclash-Splash-Site/). Unlike most implementations, BBClash is **not RegEx-based.** It functions like a compiler, tokenizing, lexing, and then constructing compliant HTML from an AST-like object. This makes it robust and good at handling even improperly-formatted input. 

Our BBCode specification can be found [here](https://github.com/EndaHallahan/BBClash/blob/master/Spec.md).

## General Usage:

```
[dependencies]
bbclash = "1.0.0"
```

```rust
use bbclash::bbcode_to_html;

assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
```

BBClash also comes ready out-of-the-box for use as WASM or with other languages via C bindings.

## Custom Usage:

Because this package was built for an existing application, and because it is performance-focused, BBClash's BBCode implementation is entirely hard-coded. Because of this, it is reccommended that you download a local copy and modify it to suit your needs. *Note: currently requires Rust Nightly to build. Relevant issue: [54727](https://github.com/rust-lang/rust/issues/54727)*

Building is as simple as running `$ cargo build`. Tests and benchmarks can be run with `$ cargo test` and `$ cargo bench`, respectively.

## License
This version of BBClash is licensed under the terms of the MIT license.