/*!
BBClash is the open-source version of the BBCode compiler being built for [Penclash](https://endahallahan.github.io/Penclash-Splash-Site/). Unlike most implementations, BBClash is **not RegEx-based.** It functions like a compiler, tokenizing, lexing, and then constructing compliant HTML from an AST-like object. This makes it robust and good at handling even improperly-formatted input. 

## General Usage:

```rust
use bbclash::bbcode_to_html;

assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
```

BBClash also comes ready out-of-the-box for use as WASM or with other languages via C bindings.

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
/// # Examples
///
/// ```
///use bbclash::bbcode_to_html;
///
///assert_eq!(bbcode_to_html("I'm [i]italic[/i] and [b]bold![/b]"), 
///		"<p>I&#x27m <i>italic</i> and <b>bold!</b></p>");
/// ```
#[no_mangle]
pub extern fn bbcode_to_html(input: &str) -> String {
    let mut tokenizer = BBCodeTokenizer::new();
	let mut lexer = BBCodeLexer::new();
	let mut constructor = HTMLConstructor::new(input.len());
	constructor.construct(lexer.lex(tokenizer.tokenize(input)))
	// Debug code:
	/*for node in out.descendants() {
		println!("{:?}", node.borrow());
	}*/
}

/// A single element of a BBCode AST.
#[derive(Debug, Clone)]
pub struct ASTElement {
	ele_type: GroupType,
	text_contents: Option<String>,
	argument: Option<String>,
	is_void: bool,
}
impl ASTElement {
	/// Creates a new ASTElement.
	pub fn new(ele_type: GroupType) -> ASTElement {
		let text_contents = None;
		let argument = None;
		let is_void = false;
		ASTElement{ele_type, text_contents, argument, is_void}
	}
	/// Sets an ASTElement's type.
	pub fn set_ele_type(&mut self, new_type: GroupType) {
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
	pub fn has_arg(&mut self) -> bool {
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
}

/// A single Instruction output by the tokenizer.
#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
	Null,
	Tag(String, Option<String>), 
	Text(String),
	Parabreak,
	Linebreak,
	Scenebreak
}

/// Types of ASTElement.
#[derive(Debug, PartialEq, Clone)]
pub enum GroupType {
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
	Opacity,
	Size,
	Center,
	Right,
	Image,
	Quote,
	//Footnote,
	//Indent,
	//Pre,
	//PreLine,
	//Header,
	//Figure,
	//List,
	//Embed,
	//Code,
	//CodeBlock,
	//Icon,
	//Math,
	//Table,
	//TableItem,
	Paragraph,
	Scenebreak,
	Null,
	Broken,
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

