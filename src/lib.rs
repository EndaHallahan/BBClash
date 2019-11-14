/*!

BBClash is the open-source version of the BBCode compiler being built for [Penclash](https://endahallahan.github.io/Penclash-Splash-Site/). Unlike most implementations, BBClash is **not RegEx-based.** It functions like a compiler, tokenizing, lexing, and then constructing compliant HTML from an AST-like object. This makes it robust and good at handling even improperly-formatted input. 

Our BBCode specification can be found [here](https://github.com/EndaHallahan/BBClash/blob/master/Spec.md).

## General Usage:

```rust
use bbclash::bbcode_to_html;

assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
```

BBClash also comes ready out-of-the-box for use as WASM or with other languages via C bindings.

## Pretty and Ugly Output

BBBClash has two main modes of operation: *pretty* and *ugly*. Pretty output uses the `bbcode_to_html` function, and excludes improperly formatted bbcode and empty elements from the final output:

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

Because this package was built for an existing application, and because it is performance-focused, BBClash's BBCode implementation is entirely hard-coded. Because of this, it is reccommended that you download a local copy and modify it to suit your needs. *Note: currently requires Rust Nightly to build. Relevant issue: [54727](https://github.com/rust-lang/rust/issues/54727)*

Building is as simple as running `$ cargo build`. Tests and benchmarks can be run with `$ cargo test` and `$ cargo bench`, respectively.
*/

#![feature(proc_macro_hygiene)]
extern crate rctree;
extern crate phf;

mod bbcode_tokenizer;
mod bbcode_lexer;
mod html_constructor;

pub use crate::bbcode_tokenizer::BBCodeTokenizer;
pub use crate::bbcode_lexer::BBCodeLexer;
pub use crate::html_constructor::HTMLConstructor;

/// Generates a string of HTML from an &str of BBCode.
/// This function produces *pretty* output, meaning that any eroneously written BBCode encountered or empty tags will be removed from the final output.
/// # Examples
///
/// ```
///use bbclash::bbcode_to_html;
///
///assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
///		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
///
///assert_eq!(bbcode_to_html("[quote][/quote]"), 
///		"");
/// ```
#[no_mangle]
pub extern fn bbcode_to_html(input: &str) -> String {
    let mut tokenizer = BBCodeTokenizer::new();
	let mut lexer = BBCodeLexer::new(false);
	let mut constructor = HTMLConstructor::new(input.len(), true);
	constructor.construct(lexer.lex(tokenizer.tokenize(input)))
}

/// Generates a string of HTML from an &str of BBCode. 
/// This function produces *ugly* output, meaning that any eroneously written BBCode or empty tags encountered will be included in the final output.
/// # Examples
///
/// ```
///use bbclash::bbcode_to_html_ugly;
///
///assert_eq!(bbcode_to_html_ugly("I'm [colour]missing an argument![/colour]"), 
///		"<p>I&#x27m [colour]missing an argument![/colour]</p>");
///
///assert_eq!(bbcode_to_html_ugly("[quote][/quote]"), 
///		"<blockquote></blockquote>");
/// ```
#[no_mangle]
pub extern fn bbcode_to_html_ugly(input: &str) -> String {
    let mut tokenizer = BBCodeTokenizer::new();
	let mut lexer = BBCodeLexer::new(true);
	let mut constructor = HTMLConstructor::new(input.len(), false);
	constructor.construct(lexer.lex(tokenizer.tokenize(input)))
}

/// A single element of a BBCode AST.
#[derive(Debug, Clone)]
pub struct ASTElement {
	ele_type: GroupType,
	text_contents: Option<String>,
	argument: Option<String>,
	is_void: bool,
	detachable: bool,
	broken: bool,
}
impl ASTElement {
	/// Creates a new ASTElement.
	pub fn new(ele_type: GroupType) -> ASTElement {
		let text_contents = None;
		let argument = None;
		let is_void = false;
		let detachable = true;
		let broken = match ele_type {
			GroupType::Broken(_,_) => true,
			_ => false
		};
		ASTElement{ele_type, text_contents, argument, is_void, detachable, broken}
	}
	/// Sets an ASTElement's type.
	pub fn set_ele_type(&mut self, new_type: GroupType) {
		self.broken = match new_type {
			GroupType::Broken(_,_) => true,
			_ => false
		};
		self.ele_type = new_type;
	}
	/// Gets an immutable reference to an ASTElement's type.
	pub fn ele_type(&self) -> &GroupType {
		&self.ele_type
	}
	/// Sets an ASTElement's is_void field (indicates that the ASTElement does not contain text or children).
	pub fn set_void(&mut self, in_void: bool) {
		self.is_void = in_void;
	}
	/// gets the value of an ASTElement's is_void field.
	pub fn is_void(&self) -> bool {
		self.is_void
	}
	/// Adds text to an ASTElement.
	pub fn add_text(&mut self, new_text: &String) {
		if let Some(text) = &self.text_contents {
			self.text_contents = Some(format!("{}{}", text, new_text));
		} else {
			self.text_contents = Some(new_text.to_string());
		}
		
	}
	/// Gets whether or not an ASTElement has text.
	pub fn has_text(&self) -> bool {
		if let Some(_) = &self.text_contents {
			true
		} else {
			false
		}
	}
	/// Gets an immutable reference to an ASTElement's text_contents.
	pub fn text_contents(&self) -> &Option<String> {
		&self.text_contents
	}
	/// Sets an ASTElement's Argument field.
	pub fn set_arg(&mut self, arg: &String) {
		self.argument = Some(arg.to_string());
	}
	/// Adds to arg of an ASTElement.
	pub fn add_arg(&mut self, new_arg: &String) {
		if let Some(arg) = &self.argument {
			self.argument = Some(format!("{}{}", arg, new_arg));
		} else {
			self.argument = Some(new_arg.to_string());
		}
	}
	/// Gets whether or not an ASTElement has an argument.
	pub fn has_arg(&self) -> bool {
		if let Some(_) = &self.argument {
			true
		} else {
			false
		}
	}
	/// Gets an immutable reference to an ASTElement's argument field.
	pub fn argument(&self) -> &Option<String> {
		&self.argument
	}
	/// Sets an ASTElement's detachable field (indicates whether the element should be detatched if empty);
	pub fn set_detachable(&mut self, in_det: bool) {
		self.detachable = in_det;
	}
	/// Gets the value of an ASTElement's detachable field.
	pub fn is_detachable(&self) -> bool {
		self.detachable
	}
	/// Gets the value of an ASTElement's broken field.
	pub fn is_broken(&self) -> bool {
		self.broken
	}
}

/// A single Instruction output by the tokenizer.
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	Null,
	Tag(String, Option<String>), 
	Text(String),
	Parabreak(String),
	Linebreak,
	Scenebreak
}

/// Types of ASTElement.
#[derive(Debug, PartialEq, Clone)]
pub enum GroupType{
	Text,
	Hr,
	Br,
	Bold,
	Strong,
	Italic,
	Emphasis,
	Underline,
	Smallcaps,
	Strikethrough,
	Monospace,
	Superscript,
	Subscript,
	Spoiler,
	Colour,
	Url,
	Email,
	Opacity,
	Size,
	Center,
	Right,
	Image,
	Quote,
	Footnote,
	Indent,
	Pre,
	PreLine,
	Header,
	Figure,
	List,
	ListItem,
	Embed,
	Code,
	CodeBlock,
	//Icon,
	Math,
	MathBlock,
	Table,
	TableRow,
	TableData,
	TableHeader,
	TableCaption,
	Paragraph,
	Scenebreak,
	Null,
	Broken(Box<GroupType>, &'static str),
	Document,
	Anchor
}

///Types of argument for Instructions.
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
	Colour(String),
	Url(String),
	Quote(String),
}

