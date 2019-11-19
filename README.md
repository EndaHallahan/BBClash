# BBClash
[![Crates.io](https://img.shields.io/crates/v/bbclash)](https://crates.io/crates/bbclash)
[![Documentation](https://docs.rs/bbclash/badge.svg)](https://docs.rs/bbclash)
[![Crates.io](https://img.shields.io/crates/l/bbclash)](https://github.com/EndaHallahan/BBClash/blob/master/LICENSE.md)

A robust, opinionated, performance-focused BBCode to HTML parser and compiler.

## What is BBClash?

BBClash is the open-source version of the BBCode compiler being built for [Penclash](https://endahallahan.github.io/Penclash-Splash-Site/). Unlike most implementations, BBClash is **not RegEx-based.** It functions like a compiler, tokenizing, lexing, and then constructing compliant HTML from an AST-like object. This makes it robust and good at handling even improperly-formatted input. 

Our BBCode specification can be found [here](https://github.com/EndaHallahan/BBClash/blob/master/Spec.md).

*Note: currently requires Rust Nightly. Relevant issue: [54727](https://github.com/rust-lang/rust/issues/54727)*

## General Usage:

```rust
use bbclash::bbcode_to_html;

assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
```

## Pretty and Ugly Output

BBClash has two main modes of operation: *pretty* and *ugly*. Pretty output uses the `bbcode_to_html` function, and excludes improperly formatted bbcode and empty elements from the final output:

```rust
use bbclash::bbcode_to_html;

assert_eq!(bbcode_to_html("I'm [colour]missing an argument![/colour]"), 
		"<p>I&#x27m missing an argument!</p>");

assert_eq!(bbcode_to_html("[quote][/quote]"), 
		"");
```

Ugly uses the `bbcode_to_html_ugly` function, and leaves improperly formatted BBCode tags and empty elements in the final output as written:

```rust
use bbclash::bbcode_to_html_ugly;

assert_eq!(bbcode_to_html_ugly("I'm [colour]missing an argument![/colour]"), 
		"<p>I&#x27m [colour]missing an argument![/colour]</p>");

assert_eq!(bbcode_to_html_ugly("[quote][/quote]"), 
		"<blockquote></blockquote>");
```

Note that neither mode arbitrarily strips any text in square brackets. This only affects improperly-written BBCode tags; `[non tags]` will not be affected.

## Custom Usage:

Because this package was built for an existing application, and because it is performance-focused, BBClash's BBCode implementation is entirely hard-coded. Because of this, it is reccommended that you download a local copy and modify it to suit your needs. 

Building is as simple as running `$ cargo build`. Tests and benchmarks can be run with `$ cargo test` and `$ cargo bench`, respectively.

## License
This version of BBClash is licensed under the terms of the MIT license.
